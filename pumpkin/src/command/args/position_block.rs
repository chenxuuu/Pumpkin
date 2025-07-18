use async_trait::async_trait;
use pumpkin_protocol::java::client::play::{ArgumentType, CommandSuggestion, SuggestionProviders};
use pumpkin_util::math::position::BlockPos;
use pumpkin_util::math::vector3::Vector3;

use crate::command::CommandSender;
use crate::command::dispatcher::CommandError;
use crate::command::tree::RawArgs;
use crate::server::Server;

use super::super::args::ArgumentConsumer;
use super::coordinate::MaybeRelativeBlockCoordinate;
use super::{Arg, DefaultNameArgConsumer, FindArg, GetClientSideArgParser};

/// x, y and z coordinates
pub struct BlockPosArgumentConsumer;

impl GetClientSideArgParser for BlockPosArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType {
        ArgumentType::BlockPos
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<SuggestionProviders> {
        None
    }
}

#[async_trait]
impl ArgumentConsumer for BlockPosArgumentConsumer {
    async fn consume<'a>(
        &'a self,
        src: &CommandSender,
        _server: &'a Server,
        args: &mut RawArgs<'a>,
    ) -> Option<Arg<'a>> {
        let pos = MaybeRelativeBlockPos::try_new(args.pop()?, args.pop()?, args.pop()?)?;

        let vec3 = pos.try_to_absolute(src.position())?;

        Some(Arg::BlockPos(vec3))
    }

    async fn suggest<'a>(
        &'a self,
        _sender: &CommandSender,
        _server: &'a Server,
        _input: &'a str,
    ) -> Result<Option<Vec<CommandSuggestion>>, CommandError> {
        Ok(None)
    }
}

struct MaybeRelativeBlockPos(
    MaybeRelativeBlockCoordinate<false>,
    MaybeRelativeBlockCoordinate<true>,
    MaybeRelativeBlockCoordinate<false>,
);

impl MaybeRelativeBlockPos {
    fn try_new(x: &str, y: &str, z: &str) -> Option<Self> {
        Some(Self(
            x.try_into().ok()?,
            y.try_into().ok()?,
            z.try_into().ok()?,
        ))
    }

    fn try_to_absolute(self, origin: Option<Vector3<f64>>) -> Option<BlockPos> {
        Some(BlockPos(Vector3::new(
            self.0.into_absolute(origin.map(|o| o.x))?,
            self.1.into_absolute(origin.map(|o| o.y))?,
            self.2.into_absolute(origin.map(|o| o.z))?,
        )))
    }
}

impl DefaultNameArgConsumer for BlockPosArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "block_pos"
    }
}

impl<'a> FindArg<'a> for BlockPosArgumentConsumer {
    type Data = BlockPos;

    fn find_arg(args: &'a super::ConsumedArgs, name: &str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::BlockPos(data)) => Ok(*data),
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}
