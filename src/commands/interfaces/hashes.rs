use crate::{
  commands,
  error::RedisError,
  interfaces::{ClientLike, RedisResult},
  types::{FromRedis, MultipleKeys, RedisKey, RedisMap, RedisValue},
};
use std::convert::TryInto;

/// Functions that implement the [Hashes](https://redis.io/commands#hashes) interface.
#[async_trait]
pub trait HashesInterface: ClientLike + Sized {
  /// Returns all fields and values of the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hgetall>
  async fn hgetall<R, K>(&self, key: K) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
  {
    into!(key);
    commands::hashes::hgetall(self, key).await?.convert()
  }

  /// Removes the specified fields from the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hdel>
  async fn hdel<R, K, F>(&self, key: K, fields: F) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<MultipleKeys> + Send,
  {
    into!(key, fields);
    commands::hashes::hdel(self, key, fields).await?.convert()
  }

  /// Returns if `field` is an existing field in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hexists>
  async fn hexists<R, K, F>(&self, key: K, field: F) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<RedisKey> + Send,
  {
    into!(key, field);
    commands::hashes::hexists(self, key, field).await?.convert()
  }

  /// Returns the value associated with `field` in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hget>
  async fn hget<R, K, F>(&self, key: K, field: F) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<RedisKey> + Send,
  {
    into!(key, field);
    commands::hashes::hget(self, key, field).await?.convert()
  }

  /// Increments the number stored at `field` in the hash stored at `key` by `increment`.
  ///
  /// <https://redis.io/commands/hincrby>
  async fn hincrby<R, K, F>(&self, key: K, field: F, increment: i64) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<RedisKey> + Send,
  {
    into!(key, field);
    commands::hashes::hincrby(self, key, field, increment).await?.convert()
  }

  /// Increment the specified `field` of a hash stored at `key`, and representing a floating point number, by the
  /// specified `increment`.
  ///
  /// <https://redis.io/commands/hincrbyfloat>
  async fn hincrbyfloat<R, K, F>(&self, key: K, field: F, increment: f64) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<RedisKey> + Send,
  {
    into!(key, field);
    commands::hashes::hincrbyfloat(self, key, field, increment)
      .await?
      .convert()
  }

  /// Returns all field names in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hkeys>
  async fn hkeys<R, K>(&self, key: K) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
  {
    into!(key);
    commands::hashes::hkeys(self, key).await?.convert()
  }

  /// Returns the number of fields contained in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hlen>
  async fn hlen<R, K>(&self, key: K) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
  {
    into!(key);
    commands::hashes::hlen(self, key).await?.convert()
  }

  /// Returns the values associated with the specified `fields` in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hmget>
  async fn hmget<R, K, F>(&self, key: K, fields: F) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<MultipleKeys> + Send,
  {
    into!(key, fields);
    commands::hashes::hmget(self, key, fields).await?.convert()
  }

  /// Sets the specified fields to their respective values in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hmset>
  async fn hmset<R, K, V>(&self, key: K, values: V) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    V: TryInto<RedisMap> + Send,
    V::Error: Into<RedisError> + Send,
  {
    into!(key);
    try_into!(values);
    commands::hashes::hmset(self, key, values).await?.convert()
  }

  /// Sets fields in the hash stored at `key` to their provided values.
  ///
  /// <https://redis.io/commands/hset>
  async fn hset<R, K, V>(&self, key: K, values: V) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    V: TryInto<RedisMap> + Send,
    V::Error: Into<RedisError> + Send,
  {
    into!(key);
    try_into!(values);
    commands::hashes::hset(self, key, values).await?.convert()
  }

  /// Sets `field` in the hash stored at `key` to `value`, only if `field` does not yet exist.
  ///
  /// <https://redis.io/commands/hsetnx>
  async fn hsetnx<R, K, F, V>(&self, key: K, field: F, value: V) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<RedisKey> + Send,
    V: TryInto<RedisValue> + Send,
    V::Error: Into<RedisError> + Send,
  {
    into!(key, field);
    try_into!(value);
    commands::hashes::hsetnx(self, key, field, value).await?.convert()
  }

  /// When called with just the `key` argument, return a random field from the hash value stored at `key`.
  ///
  /// If the provided `count` argument is positive, return an array of distinct fields.
  ///
  /// <https://redis.io/commands/hrandfield>
  async fn hrandfield<R, K>(&self, key: K, count: Option<(i64, bool)>) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
  {
    into!(key);
    commands::hashes::hrandfield(self, key, count).await?.convert()
  }

  /// Returns the string length of the value associated with `field` in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hstrlen>
  async fn hstrlen<R, K, F>(&self, key: K, field: F) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
    F: Into<RedisKey> + Send,
  {
    into!(key, field);
    commands::hashes::hstrlen(self, key, field).await?.convert()
  }

  /// Returns all values in the hash stored at `key`.
  ///
  /// <https://redis.io/commands/hvals>
  async fn hvals<R, K>(&self, key: K) -> RedisResult<R>
  where
    R: FromRedis,
    K: Into<RedisKey> + Send,
  {
    into!(key);
    commands::hashes::hvals(self, key).await?.convert()
  }
}
