#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Node Node;
typedef struct Publisher Publisher;
typedef struct String String;
typedef struct StringVec StringVec;

// Node
Node* nodeCreate(const char* partition);
void nodeDestroy(Node**);

// Topic
StringVec* nodeTopicList(const Node*);

// Topic Pub
StringVec* nodeAdvertisedTopics(const Node*);
Publisher* nodeAdvertise(Node* node, const char* topic, const char* msgType);
bool publisherPublish(Publisher* pub, const char* msg, size_t msg_len);
void publisherDestroy(Publisher**);

// Topic Sub
StringVec* nodeSubscribedTopics(const Node* node);
bool nodeSubscribe(Node* node,
                   const char* topic,
                   void (*_callback)(const char*, size_t, const char*, void*),
                   void* userData);
bool nodeUnsubscribe(Node* node, const char* topic);

// Service
StringVec* nodeServiceList(const Node*);

// Service Client
bool nodeRequest(Node* node,
                 const char* topic,
                 const char* req,
                 size_t req_len,
                 const char* reqtype,
                 const char* restype,
                 unsigned int timeout,
                 String* res,
                 bool* result);

String* stringCreate();
size_t stringLength(const String* str);
const char* stringGet(const String*);
void stringDestroy(String**);

StringVec* stringVecCreate();
size_t stringVecSize(const StringVec*);
const char* stringVecAt(const StringVec*, size_t);
void stringVecDestroy(StringVec**);

void waitForShutdown();

#ifdef __cplusplus
}
#endif
