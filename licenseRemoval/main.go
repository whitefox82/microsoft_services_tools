package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/joho/godotenv"
	"golang.org/x/oauth2/clientcredentials"
)

// Structs for parsing JSON responses
type LicenseDetails struct {
	SkuID string `json:"skuId"`
}

type AssignedLicensesResponse struct {
	Value []LicenseDetails `json:"value"`
}

type RemoveLicensesRequest struct {
	AddLicenses    []interface{} `json:"addLicenses"`
	RemoveLicenses []string      `json:"removeLicenses"`
}

// Config holds the application configuration
type Config struct {
	TenantID     string
	ClientID     string
	ClientSecret string
}

// Client represents the Microsoft Graph API client
type Client struct {
	httpClient *http.Client
	baseURL    string
}

// NewClient creates a new Microsoft Graph API client
func NewClient(config Config) (*Client, error) {
	ctx := context.Background()
	conf := &clientcredentials.Config{
		ClientID:     config.ClientID,
		ClientSecret: config.ClientSecret,
		TokenURL:     fmt.Sprintf("https://login.microsoftonline.com/%s/oauth2/v2.0/token", config.TenantID),
		Scopes:       []string{"https://graph.microsoft.com/.default"},
	}

	httpClient := conf.Client(ctx)

	return &Client{
		httpClient: httpClient,
		baseURL:    "https://graph.microsoft.com/v1.0",
	}, nil
}

// GetAssignedLicenses fetches all assigned licenses (skuIds) for a user
func (c *Client) GetAssignedLicenses(userPrincipalName string) ([]string, error) {
	url := fmt.Sprintf("%s/users/%s/assignedLicenses", c.baseURL, userPrincipalName)

	resp, err := c.httpClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to get assigned licenses: %w", err)
	}
	defer resp.Body.Close()

	var licensesResponse AssignedLicensesResponse
	if err := json.NewDecoder(resp.Body).Decode(&licensesResponse); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	var skuIDs []string
	for _, license := range licensesResponse.Value {
		skuIDs = append(skuIDs, license.SkuID)
	}

	return skuIDs, nil
}

// RemoveLicenses removes the specified licenses from a user
func (c *Client) RemoveLicenses(userPrincipalName string, skuIDs []string) error {
	url := fmt.Sprintf("%s/users/%s/assignLicense", c.baseURL, userPrincipalName)

	requestBody := RemoveLicensesRequest{
		AddLicenses:    []interface{}{},
		RemoveLicenses: skuIDs,
	}

	jsonBody, err := json.Marshal(requestBody)
	if err != nil {
		return fmt.Errorf("failed to marshal request body: %w", err)
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonBody))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Add("Content-Type", "application/json")

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	return nil
}

func loadConfig() (Config, error) {
	if err := godotenv.Load(); err != nil {
		return Config{}, fmt.Errorf("error loading .env file: %w", err)
	}

	config := Config{
		TenantID:     os.Getenv("TENANT_ID"),
		ClientID:     os.Getenv("CLIENT_ID"),
		ClientSecret: os.Getenv("CLIENT_SECRET"),
	}

	if config.TenantID == "" || config.ClientID == "" || config.ClientSecret == "" {
		return Config{}, fmt.Errorf("missing required environment variables")
	}

	return config, nil
}

func main() {
	if len(os.Args) < 2 {
		log.Fatal("Usage: go run . <userPrincipalName>")
	}
	userPrincipalName := os.Args[1]

	config, err := loadConfig()
	if err != nil {
		log.Fatalf("Failed to load configuration: %v", err)
	}

	client, err := NewClient(config)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	skuIDs, err := client.GetAssignedLicenses(userPrincipalName)
	if err != nil {
		log.Fatalf("Error getting assigned licenses: %v", err)
	}

	if len(skuIDs) == 0 {
		fmt.Println("No licenses found to remove.")
		return
	}

	if err := client.RemoveLicenses(userPrincipalName, skuIDs); err != nil {
		log.Fatalf("Error removing licenses: %v", err)
	}

	fmt.Println("Licenses successfully removed.")
}
