use super::*;
use crate::{
  interfaces,
  protocol::{
    command::{RouterCommand, RedisCommand, RedisCommandKind},
    utils as protocol_utils,
  },
  types::*,
  utils,
};
use bytes_utils::Str;
use tokio::sync::oneshot::channel as oneshot_channel;

value_cmd!(client_id, ClientID);
value_cmd!(client_info, ClientInfo);

pub async fn client_kill<C: ClientLike>(
  client: &C,
  filters: Vec<ClientKillFilter>,
) -> Result<RedisValue, RedisError> {
  let frame = utils::request_response(client, move || {
    let mut args = Vec::with_capacity(filters.len() * 2);

    for filter in filters.into_iter() {
      let (field, value) = filter.to_str();
      args.push(field.into());
      args.push(value.into());
    }

    Ok((RedisCommandKind::ClientKill, args))
  })
  .await?;

  protocol_utils::frame_to_single_result(frame)
}

pub async fn client_list<C: ClientLike>(
  client: &C,
  r#type: Option<ClientKillType>,
  ids: Option<Vec<String>>,
) -> Result<RedisValue, RedisError> {
  let ids: Option<Vec<RedisKey>> = ids.map(|ids| ids.into_iter().map(|id| id.into()).collect());
  let frame = utils::request_response(client, move || {
    let max_args = 2 + ids.as_ref().map(|i| i.len()).unwrap_or(0);
    let mut args = Vec::with_capacity(max_args);

    if let Some(kind) = r#type {
      args.push(static_val!(TYPE));
      args.push(kind.to_str().into());
    }
    if let Some(ids) = ids {
      if !ids.is_empty() {
        args.push(static_val!(ID));

        for id in ids.into_iter() {
          args.push(id.into());
        }
      }
    }

    Ok((RedisCommandKind::ClientList, args))
  })
  .await?;

  protocol_utils::frame_to_single_result(frame)
}

pub async fn client_pause<C: ClientLike>(
  client: &C,
  timeout: i64,
  mode: Option<ClientPauseKind>,
) -> Result<(), RedisError> {
  let frame = utils::request_response(client, move || {
    let mut args = Vec::with_capacity(2);
    args.push(timeout.into());

    if let Some(mode) = mode {
      args.push(mode.to_str().into());
    }

    Ok((RedisCommandKind::ClientPause, args))
  })
  .await?;

  let response = protocol_utils::frame_to_single_result(frame)?;
  protocol_utils::expect_ok(&response)
}

value_cmd!(client_getname, ClientGetName);

pub async fn client_setname<C: ClientLike>(client: &C, name: Str) -> Result<(), RedisError> {
  let inner = client.inner();
  _warn!(
    inner,
    "Changing client name from {} to {}",
    client.inner().id.as_str(),
    name
  );

  let frame =
    utils::request_response(client, move || Ok((RedisCommandKind::ClientSetname, vec![name.into()]))).await?;
  let response = protocol_utils::frame_to_single_result(frame)?;
  protocol_utils::expect_ok(&response)
}

ok_cmd!(client_unpause, ClientUnpause);

pub async fn client_reply<C: ClientLike>(client: &C, flag: ClientReplyFlag) -> Result<(), RedisError> {
  let frame = utils::request_response(client, move || {
    Ok((RedisCommandKind::ClientReply, vec![flag.to_str().into()]))
  })
  .await?;

  let response = protocol_utils::frame_to_single_result(frame)?;
  protocol_utils::expect_ok(&response)
}

pub async fn client_unblock<C: ClientLike>(
  client: &C,
  id: RedisValue,
  flag: Option<ClientUnblockFlag>,
) -> Result<RedisValue, RedisError> {
  let inner = client.inner();

  let mut args = Vec::with_capacity(2);
  args.push(id);
  if let Some(flag) = flag {
    args.push(flag.to_str().into());
  }
  let command = RedisCommand::new(RedisCommandKind::ClientUnblock, args);

  let frame = utils::backchannel_request_response(inner, command, false).await?;
  protocol_utils::frame_to_single_result(frame)
}

pub async fn unblock_self<C: ClientLike>(client: &C, flag: Option<ClientUnblockFlag>) -> Result<(), RedisError> {
  let inner = client.inner();
  let flag = flag.unwrap_or(ClientUnblockFlag::Error);
  let result = utils::interrupt_blocked_connection(inner, flag).await;
  inner.backchannel.write().await.set_unblocked();
  result
}

pub async fn active_connections<C: ClientLike>(client: &C) -> Result<Vec<Server>, RedisError> {
  let (tx, rx) = oneshot_channel();
  let command = RouterCommand::Connections { tx };
  let _ = interfaces::send_to_router(client.inner(), command)?;

  rx.await.map_err(|e| e.into())
}
