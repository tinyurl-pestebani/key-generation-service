# key-generation-service

This service is responsible for generating unique keys for shortened URLs. It provides gRPC endpoints for key generation. Its gRPC definition can be found at `https://github.com/tinyurl-pestebani/proto/blob/main/v1/key-generator.proto`.


## Generator module

This module provides different implementations of key generators. The key generation provides a trait `KeyGenerator` that defines the interface for generating keys (8 alphanumeric characters). The following implementations are available:

- `RandomGenerator`: Generates keys using a random alphanumeric string generator.
- `RedisGenerator`: Generates keys using a Redis instance and an autoincrement key to ensure uniqueness.
- `PrimitiveRootRedisGenerator`: Generates keys using a Redis instance and a primitive root algorithm to ensure uniqueness and better distribution.
For using this algorithm, you need a huge prime number, and one primitive root of that number.
  - **Generating the prime number**: We need a prime number lower than the maximum allowed (56^8). We can use the following Python code: 
  ```python
  from Crypto.Util import number
  # 9.67 * 1e13 is approx 2^46
  n = number.getPrime(46)
  # n = 37845836980717
  print(n)
  ```
  - **Finding a primitive root**: For finding a primitive root, use libraries such as [sympy](https://docs.sympy.org/).


## Environment Variables
The service requires the following environment variables to be set:
- `GENERATION_KEY_SERVICE_PORT`: The port on which the service will run (default: `8080`).
- `GENERATOR_TYPE`: The type of key generator to use. Possible values are `random`, `redis`, and `primitive_root_redis` (default: `random`).
- `REDIS_URL`: The Redis server URL (default: `redis://localhost:6379`).
- `GENERATOR_PRIME`: The prime number to use for the `PrimitiveRootRedisGenerator` (default: `1000003`).
- `GENERATOR_INCREMENT_START`: The starting value for the autoincrement counter in `PrimitiveRootRedisGenerator` (default: `0`).
- `GENERATOR_PRIME_PRIMITIVE`: Prime number primitive root to use for the `PrimitiveRootRedisGenerator` (default: `2`).
- `NUMBER_DIGITS`: The number of digits to use for the key generation (default: `8`).

For OpenTelemetry configuration, please refer to the [OpenTelemetry setup repository](https://github.com/tinyurl-pestebani/rust-otel-setup).
