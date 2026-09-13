#include <gz/transport/Node.hh>
#include <gz/msgs/stringmsg.pb.h>

#include <atomic>
#include <functional>
#include <memory>

struct TestService {
  // Destroy the node before the callback's shared counter.
  std::shared_ptr<std::atomic<unsigned int>> calls =
      std::make_shared<std::atomic<unsigned int>>(0);
  std::unique_ptr<gz::transport::Node> node;
};

extern "C" TestService* testServiceCreate(const char* partition) {
  gz::transport::NodeOptions options;
  if (!options.SetPartition(partition)) {
    return nullptr;
  }
  auto service = std::make_unique<TestService>();
  service->node = std::make_unique<gz::transport::Node>(options);
  std::function<bool(const gz::msgs::StringMsg&, gz::msgs::StringMsg&)> echo =
      [calls = service->calls](const gz::msgs::StringMsg& req,
                               gz::msgs::StringMsg& res) {
        ++*calls;
        res = req;
        return true;
      };
  if (!service->node->Advertise("/echo", echo)) {
    return nullptr;
  }
  return service.release();
}

extern "C" unsigned int testServiceCalls(const TestService* service) {
  return service->calls->load();
}

extern "C" void testServiceDestroy(TestService* service) {
  delete service;
}
