namespace Aiot.Sdk
{
    public class AiotClient
    {
        private string _endpoint;

        public AiotClient(string endpoint)
        {
            _endpoint = endpoint;
        }

        public bool Ping()
        {
            return true;
        }

        public bool Start()
        {
            return true;
        }

        public bool Stop()
        {
            return true;
        }
    }
}
