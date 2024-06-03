# Web Servers

The two main protocols involved in web servers are `Hypertext Transfer Protocol (HTTP)`
and `Transmission Control Protocol (TCP)`.

Both `protocols` are `request-response protocols`, meaning a `client initiates requests` and
a `server listens to the requests and provides a response to the client`.

The contents of those requests and responses are defined by the protocols.

## TCP

TCP is the lower-level protocol that describes the details of how information gets from one server to another but does
not specify what that information is.

## HTTP

HTTP builds on top of TCP by defining the contents of the requests and responses.

It’s technically possible to use HTTP with other protocols, but in the vast majority of cases, HTTP sends its data over
TCP.

## Stream

A single stream represents an `open connection` between the client and the server.

## Connection

A `connection` is the name for the `full request and response process` in which a client connects to the server, the
server generates a response, and the server closes the connection.

## webserver design techniques

- `thread pool`
- `fork/join model`,
- `single-threaded async I/O model`,
- `multithreaded async I/O model`.

### Techniques for improving the efficiency of a webserver

1. **Asynchronous Processing**:
    - **Non-blocking I/O**: Use non-blocking I/O operations to handle multiple requests concurrently without waiting for
      I/O operations to complete.
    - **Event-driven Architecture**: Implement an event-driven architecture where an event loop can handle multiple
      tasks by switching between them as they become ready, avoiding idle wait times.
    - **Callback Functions and Promises/Futures**: Use callback functions, promises, or futures to handle the completion
      of asynchronous operations without blocking the main thread.

2. **Thread Pool**:
    - Use a thread pool to manage a pool of worker threads, allowing multiple requests to be processed concurrently.
      This helps to balance the load and avoid any single request from monopolizing resources.

3. **Load Balancing**:
    - **Round-robin Load Balancing**: Distribute incoming requests evenly across multiple servers.
    - **Least Connections Load Balancing**: Direct new requests to the server with the fewest active connections.
    - **Resource-based Load Balancing**: Allocate requests based on server resource usage (CPU, memory, etc.).

4. **Message Queues**:
    - **Job Queues**: Use message queuing systems (like RabbitMQ, Kafka, or AWS SQS) to decouple the production and
      consumption of tasks. Producers send tasks to a queue, and consumers (workers) process these tasks at their own
      pace.
    - **Priority Queues**: Implement priority queues to handle high-priority requests before low-priority ones.

5. **Circuit Breaker Pattern**:
    - Implement circuit breakers to detect failures and short-circuit requests to failing services, providing fallback
      responses or alternative actions to maintain system responsiveness.

6. **Timeouts and Retries**:
    - **Timeouts**: Set appropriate timeouts for requests to prevent them from hanging indefinitely.
    - **Retries**: Implement retry logic with exponential backoff to handle transient failures gracefully without
      overwhelming the system.

7. **Caching**:
    - **In-memory Caching**: Use in-memory caches (like Redis or Memcached) to store frequently accessed data, reducing
      the need for repeated, slow database queries.
    - **HTTP Caching**: Utilize HTTP caching headers (e.g., ETag, Cache-Control) to allow clients and intermediate
      proxies to cache responses.

8. **Rate Limiting and Throttling**:
    - Implement rate limiting to control the number of requests a client can make in a given time period, preventing
      overloading of the system.
    - Throttle requests to ensure that resources are used within safe limits.

9. **Microservices Architecture**:
    - Break down a monolithic application into microservices, allowing individual services to scale independently and
      handle requests in parallel.

10. **Vertical and Horizontal Scaling**:
    - **Vertical Scaling**: Increase the capacity of existing servers by adding more resources (CPU, memory, etc.).
    - **Horizontal Scaling**: Add more servers to handle increased load, distributing requests across a larger pool of
      resources.

11. **Bulkheads**:
    - Isolate critical services from less critical ones to prevent failures in one part of the system from impacting
      others.

Implementing a combination of these techniques can significantly enhance system performance and reliability, ensuring
that slow requests do not create bottlenecks and affect overall responsiveness.


