use crate::loc_data;

#[allow(unused)]
pub(crate) struct LocationData<'a> {
    region: &'a str,
    shard: &'a str,
}

pub(crate) enum Location {
    Europe,
    NorthAmerica,
    LatinAmerica,
    Brazil,
    Korea,
    AsiaPacific,
    PBE,
}

impl Location {
    fn data(&self) -> LocationData {
        match *self {
            Self::Europe => loc_data!("eu", "eu"),
            Self::NorthAmerica => loc_data!("na", "na"),
            Self::LatinAmerica => loc_data!("latam", "na"),
            Self::Brazil => loc_data!("br", "na"),
            Self::Korea => loc_data!("kr", "kr"),
            Self::AsiaPacific => loc_data!("ap", "ap"),
            Self::PBE => loc_data!("na", "pbe"),
        }
    }
}

#[macro_export]
macro_rules! loc_data {
    ($region:literal, $shard:literal) => {
        LocationData {
            region: $region,
            shard: $shard,
        }
    };
}
