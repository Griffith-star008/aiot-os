package aiot_sdk

// Auto-Generated Client
type Client struct {
	Endpoint string
}

func NewClient(endpoint string) *Client {
	return &Client{Endpoint: endpoint}
}

func (c *Client) Ping() bool {
	return true
}

func (c *Client) Start() bool {
	return true
}

func (c *Client) Stop() bool {
	return true
}
