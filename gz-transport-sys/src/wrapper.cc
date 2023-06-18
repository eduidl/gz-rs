#include "wrapper.h"

#include <gz/transport/Node.hh>

#include <memory>

struct Node {
  std::unique_ptr<gz::transport::Node> impl;
};

struct Publisher {
  gz::transport::Node::Publisher impl;
  std::string msgType;
};

struct String {
  std::string inner;
};

struct StringVec {
  std::vector<std::string> inner;
};

template <typename T>
void destroyPtr(T** ptr) {
  if (*ptr != nullptr) {
    delete *ptr;
    *ptr = nullptr;
  }
}

Node* nodeCreate(const char* partition) {
  auto node = new Node();
  gz::transport::NodeOptions ops;
  if (partition != nullptr) {
    if (!ops.SetPartition(partition)) {
      delete node;
      return nullptr;
    }
  }
  node->impl = std::make_unique<gz::transport::Node>(ops);
  return node;
}

void nodeDestroy(Node** node) {
  destroyPtr(node);
}

StringVec* nodeTopicList(const Node* node) {
  auto vec = new StringVec();
  node->impl->TopicList(vec->inner);
  return vec;
}

StringVec* nodeAdvertisedTopics(const Node* node) {
  auto vec = new StringVec();
  vec->inner = node->impl->AdvertisedTopics();
  return vec;
}

Publisher* nodeAdvertise(Node* node, const char* topic, const char* msgType) {
  auto pub = new Publisher();
  pub->impl = node->impl->Advertise(topic, msgType);
  if (!pub->impl.Valid()) {
    delete pub;
    return nullptr;
  }
  pub->msgType = msgType;
  return pub;
}

bool publisherPublish(Publisher* pub, const char* msg, size_t msg_len) {
  std::string msg_str(msg, msg_len);
  return pub->impl.PublishRaw(msg_str, pub->msgType);
}

void publisherDestroy(Publisher** pub) {
  if (*pub != nullptr) {
    delete *pub;
    *pub = nullptr;
  }
}

StringVec* nodeSubscribedTopics(const Node* node) {
  auto vec = new StringVec();
  vec->inner = node->impl->SubscribedTopics();
  return vec;
}

bool nodeSubscribe(Node* node,
                   const char* topic,
                   void (*_callback)(const char*, size_t, const char*, void*),
                   void* userData) {
  using gz::transport::MessageInfo;

  const auto callback = [_callback, userData](const char* msg,
                                              const size_t size,
                                              const MessageInfo& info) {
    _callback(msg, size, info.Type().c_str(), userData);
  };

  return node->impl->SubscribeRaw(topic, callback);
}

bool nodeUnsubscribe(Node* node, const char* topic) {
  return node->impl->Unsubscribe(topic);
}

StringVec* nodeServiceList(const Node* node) {
  auto vec = new StringVec();
  node->impl->ServiceList(vec->inner);
  return vec;
}

bool nodeRequest(Node* node,
                 const char* topic,
                 const char* _req,
                 const char* reqtype,
                 const char* restype,
                 unsigned int timeout,
                 String* _res,
                 bool* result) {
  // TODO: Replace with official implemenation
  std::unique_ptr<google::protobuf::Message> req =
      gz::msgs::Factory::New(reqtype);
  if (!req) {
    return false;
  }
  req->ParseFromString(_req);

  std::unique_ptr<google::protobuf::Message> res =
      gz::msgs::Factory::New(restype);
  if (!res) {
    return false;
  }

  bool executed = node->impl->Request(topic, *req, timeout, *res, *result);
  return executed && res->SerializeToString(&_res->inner);
}

String* stringCreate() {
  auto str = new String();
  str->inner = std::string();
  return str;
}

size_t stringLength(const String* str) {
  return str->inner.length();
}

const char* stringGet(const String* str) {
  return str->inner.c_str();
}

void stringDestroy(String** str) {
  destroyPtr(str);
}

StringVec* stringVecCreate() {
  auto vec = new StringVec();
  vec->inner = std::vector<std::string>();
  return vec;
}

size_t stringVecSize(const StringVec* vec) {
  return vec->inner.size();
}

const char* stringVecAt(const StringVec* vec, size_t index) {
  return vec->inner.at(index).c_str();
}

void stringVecDestroy(StringVec** vec) {
  destroyPtr(vec);
}

void waitForShutdown() {
  gz::transport::waitForShutdown();
}
