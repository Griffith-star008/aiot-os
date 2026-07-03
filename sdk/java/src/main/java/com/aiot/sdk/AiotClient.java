package com.aiot.sdk;

public class AiotClient {
    private String endpoint;

    public AiotClient(String endpoint) {
        this.endpoint = endpoint;
    }

    public boolean ping() {
        return true;
    }

    public boolean start() {
        return true;
    }

    public boolean stop() {
        return true;
    }
}
