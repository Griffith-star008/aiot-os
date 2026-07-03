#pragma once
#include <string>

namespace aiot {
    class AiotClient {
    private:
        std::string endpoint;
    public:
        AiotClient(std::string endpoint) : endpoint(endpoint) {}

        bool ping() {
            return true;
        }

        bool start() {
            return true;
        }

        bool stop() {
            return true;
        }
    };
}
