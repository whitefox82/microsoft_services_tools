package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/Azure/azure-sdk-for-go/sdk/azidentity"
	"github.com/Azure/azure-sdk-for-go/sdk/resourcemanager/commerce/armcommerce"
	"github.com/joho/godotenv"
)

type SkuAvailability struct {
	SkuPartNumber  string `json:"skuPartNumber"`
	RemainingUnits int    `json:"remainingUnits"`
}

func main() {
	if err := run(); err != nil {
		log.Fatalf("Error: %v", err)
	}
}

func run() error {
	skuPartNumbers := parseArgs()
	config, err := loadConfig()
	if err != nil {
		return fmt.Errorf("failed to load config: %w", err)
	}

	client, err := newCommerceClient(config)
	if err != nil {
		return fmt.Errorf("failed to create commerce client: %w", err)
	}

	availabilities, err := getSkuAvailabilities(client, skuPartNumbers)
	if err != nil {
		return fmt.Errorf("failed to get SKU availabilities: %w", err)
	}

	return printJSON(availabilities)
}

func parseArgs() []string {
	if len(os.Args) < 2 {
		log.Println("Usage: go run . <SKU_PART_NUMBER_1> <SKU_PART_NUMBER_2> ... | * for all SKUs")
		os.Exit(1)
	}
	return os.Args[1:]
}

type config struct {
	ClientID     string
	TenantID     string
	ClientSecret string
}

func loadConfig() (*config, error) {
	if err := godotenv.Load(); err != nil {
		return nil, fmt.Errorf("error loading .env file: %w", err)
	}

	cfg := &config{
		ClientID:     os.Getenv("CLIENT_ID"),
		TenantID:     os.Getenv("TENANT_ID"),
		ClientSecret: os.Getenv("CLIENT_SECRET"),
	}

	if cfg.ClientID == "" || cfg.TenantID == "" || cfg.ClientSecret == "" {
		return nil, fmt.Errorf("missing required environment variables")
	}

	return cfg, nil
}

func newCommerceClient(cfg *config) (*armcommerce.UsageAggregatesClient, error) {
	cred, err := azidentity.NewClientSecretCredential(cfg.TenantID, cfg.ClientID, cfg.ClientSecret, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create credential: %w", err)
	}

	client, err := armcommerce.NewUsageAggregatesClient(cfg.TenantID, cred, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create commerce client: %w", err)
	}

	return client, nil
}

func getSkuAvailabilities(client *armcommerce.UsageAggregatesClient, skuPartNumbers []string) ([]SkuAvailability, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	pager := client.NewListPager(nil)

	var availabilities []SkuAvailability
	for pager.More() {
		page, err := pager.NextPage(ctx)
		if err != nil {
			return nil, fmt.Errorf("failed to list SKUs: %w", err)
		}

		for _, sku := range page.Value {
			if shouldIncludeSku(skuPartNumbers, *sku.Name) {
				remaining := int(*sku.Capacity - *sku.UsedCapacity)
				availabilities = append(availabilities, SkuAvailability{
					SkuPartNumber:  *sku.Name,
					RemainingUnits: remaining,
				})
			}
		}
	}

	return availabilities, nil
}

func shouldIncludeSku(skuPartNumbers []string, skuPartNumber string) bool {
	return len(skuPartNumbers) == 1 && skuPartNumbers[0] == "*" ||
		contains(skuPartNumbers, skuPartNumber)
}

func contains(slice []string, str string) bool {
	for _, v := range slice {
		if v == str {
			return true
		}
	}
	return false
}

func printJSON(data interface{}) error {
	output, err := json.Marshal(data)
	if err != nil {
		return fmt.Errorf("error marshaling results: %w", err)
	}
	fmt.Println(string(output))
	return nil
}
