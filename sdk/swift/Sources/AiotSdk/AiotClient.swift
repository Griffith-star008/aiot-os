public class AiotClient {
    private let endpoint: String

    public init(endpoint: String) {
        self.endpoint = endpoint
    }

    public func ping() -> Bool {
        return true
    }

    public func start() -> Bool {
        return true
    }

    public func stop() -> Bool {
        return true
    }
}
