use std::{
    fs::{create_dir_all, read_dir, remove_file, DirEntry, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    entities::{asset::Asset, pair::Pair, pair_group::PairGroup, tag::Tag, watchlist::Watchlist},
    implementations::data_access::file_system::file_system_pair::FileSystemPair,
    interactors::{
        delete_asset::DeleteAssetDataAccess, delete_pair_group::DeletePairGroupDataAccess,
        delete_tag::DeleteTagDataAccess, delete_watchlist_pair::DeleteWatchlistPairDataAccess,
        save_pair_group::SavePairGroupDataAccess, save_tag::SaveTagDataAccess,
        store_portfolios::StorePortfoliosDataAccess,
        store_watchlist_coins::StoreWatchlistCoinsDataAccess,
        update_pair_group::UpdatePairGroupDataAccess, update_portfolio::UpdatePortfolioDataAccess,
        view_pair_groups::ViewPairGroupsDataAccess, view_portfolios::ViewPortfoliosDataAccess,
        view_watchlist::ViewWatchlistDataAccess,
    },
    Error,
};

use super::{
    file_system_asset::FileSystemAsset, file_system_pair_group::FileSystemPairGroup,
    file_system_tag::FileSystemTag, file_system_watchlist::FileSystemWatchlist,
};

const TAGS_DIR_NAME: &str = "tag";
const PAIRS_DIR_NAME: &str = "pairs";
const ASSETS_DIR_NAME: &str = "assets";
const WATCHLISTS_DIR_NAME: &str = "watchlists";
const PAIR_GROUPS_DIR_NAME: &str = "pair_groups";

pub struct FileSystemDataAccess {
    pub root: PathBuf,
}

impl ViewPairGroupsDataAccess for FileSystemDataAccess {
    async fn update_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        return update_pair(&self, pair).await;
    }

    async fn fetch_pair_groups(&mut self) -> Result<Vec<PairGroup>, Error> {
        return fetch_pair_groups(&self).await;
    }

    async fn update_pair_group(&mut self, pair_group: &PairGroup) -> Result<(), Error> {
        return update_pair_group(&self, pair_group).await;
    }
}

async fn update_pair(data_access: &FileSystemDataAccess, pair: &Pair) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, PAIRS_DIR_NAME)?;
    let path = dir.join(&pair.id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Pair to update does not exist!"),
        });
    }
    write_pair(&data_access.root, pair)?;
    return Ok(());
}

fn ensure_dir(root: &Path, name: &str) -> Result<PathBuf, Error> {
    let dir = root.join(name);
    create_dir_all(&dir).expect("Could not create database directory!");
    return Ok(dir);
}

fn write_pair(root: &Path, pair: &Pair) -> Result<(), Error> {
    let dir = ensure_dir(root, PAIRS_DIR_NAME)?;
    let path = dir.join(&pair.id);
    write_object_file(
        &path,
        &FileSystemPair {
            id: pair.id.clone(),
            base: pair.base.clone(),
            value: pair.value.clone(),
            comparison: pair.comparison.clone(),
            created_at: pair.created_at.clone(),
            updated_at: pair.updated_at.clone(),
        },
    )?;
    return Ok(());
}

fn write_object_file<T>(path: &Path, object: &T) -> Result<(), Error>
where
    T: for<'a> Serialize,
{
    let object_contents = serde_json::to_string(object).map_err(|e| Error {
        message: e.to_string(),
    })?;
    File::create(path)
        .and_then(|mut file| file.write_all(object_contents.as_bytes()))
        .map_err(|e| Error {
            message: e.to_string(),
        })?;
    return Ok(());
}

async fn fetch_pair_groups(data_access: &FileSystemDataAccess) -> Result<Vec<PairGroup>, Error> {
    let mut pair_groups: Vec<PairGroup> = vec![];
    let entries = get_dir_entries(&data_access.root, PAIR_GROUPS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(id) = file_name.to_str() {
            let pair_group = read_pair_group(&data_access.root, id)?;
            pair_groups.push(pair_group);
        }
    }
    return Ok(pair_groups);
}

fn get_dir_entries(root: &Path, name: &str) -> Result<Vec<DirEntry>, Error> {
    let mut dir_entries: Vec<DirEntry> = vec![];
    let dir = ensure_dir(root, name)?;
    let dir_entry_results = read_dir(&dir).map_err(|e| Error {
        message: e.to_string(),
    })?;
    for dir_entry_result in dir_entry_results {
        let dir_entry = dir_entry_result.map_err(|e| Error {
            message: e.to_string(),
        })?;
        dir_entries.push(dir_entry);
    }
    return Ok(dir_entries);
}

fn read_pair_group(root: &Path, id: &str) -> Result<PairGroup, Error> {
    let dir = ensure_dir(root, PAIR_GROUPS_DIR_NAME)?;
    let path = dir.join(id);
    let fs_pair_group = create_object_from_file::<FileSystemPairGroup>(&path)?;
    let mut pair_group = PairGroup {
        id: fs_pair_group.id.clone(),
        pairs: vec![],
        is_pinned: fs_pair_group.is_pinned,
        multiplier: fs_pair_group.multiplier,
        created_at: fs_pair_group.created_at.clone(),
        updated_at: fs_pair_group.updated_at.clone(),
    };
    for pair_id in &fs_pair_group.pairs {
        let pair = read_pair(root, &pair_id)?;
        pair_group.pairs.push(pair);
    }
    return Ok(pair_group);
}

fn create_object_from_file<T>(path: &Path) -> Result<T, Error>
where
    T: for<'a> Deserialize<'a>,
{
    let mut file = File::open(path).map_err(|e| Error {
        message: e.to_string(),
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| Error {
        message: e.to_string(),
    })?;
    let object = serde_json::from_str::<T>(&contents).map_err(|e| Error {
        message: e.to_string(),
    })?;
    return Ok(object);
}

fn read_pair(root: &Path, id: &str) -> Result<Pair, Error> {
    let dir = ensure_dir(root, PAIRS_DIR_NAME)?;
    let path = dir.join(id);
    let fs_pair = create_object_from_file::<FileSystemPair>(&path)?;
    return Ok(Pair {
        id: fs_pair.id.clone(),
        base: fs_pair.base.clone(),
        value: fs_pair.value.clone(),
        comparison: fs_pair.comparison.clone(),
        created_at: fs_pair.created_at.clone(),
        updated_at: fs_pair.updated_at.clone(),
    });
}

async fn update_pair_group(
    data_access: &FileSystemDataAccess,
    pair_group: &PairGroup,
) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, PAIR_GROUPS_DIR_NAME)?;
    let path = dir.join(&pair_group.id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Pair group to update does not exist!"),
        });
    }
    write_pair_group(&data_access.root, pair_group)?;
    return Ok(());
}

fn write_pair_group(root: &Path, pair_group: &PairGroup) -> Result<(), Error> {
    let dir = ensure_dir(root, PAIR_GROUPS_DIR_NAME)?;
    let path = dir.join(&pair_group.id);
    write_object_file(
        &path,
        &FileSystemPairGroup {
            id: pair_group.id.clone(),
            is_pinned: pair_group.is_pinned,
            multiplier: pair_group.multiplier,
            pairs: pair_group.pairs.iter().map(|p| p.id.clone()).collect(),
            created_at: pair_group.created_at.clone(),
            updated_at: pair_group.updated_at.clone(),
        },
    )?;
    return Ok(());
}

impl SavePairGroupDataAccess for FileSystemDataAccess {
    async fn save_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        return save_pair(&self, pair).await;
    }

    async fn save_pair_group(&mut self, pair_group: &PairGroup) -> Result<(), Error> {
        return save_pair_group(&self, pair_group).await;
    }
}

async fn save_pair(data_access: &FileSystemDataAccess, pair: &Pair) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, PAIRS_DIR_NAME)?;
    let path = dir.join(&pair.id);
    if path.exists() {
        return Err(Error {
            message: String::from("Pair to save already exists!"),
        });
    }
    write_pair(&data_access.root, pair)?;
    return Ok(());
}

async fn save_pair_group(
    data_access: &FileSystemDataAccess,
    pair_group: &PairGroup,
) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, PAIR_GROUPS_DIR_NAME)?;
    let path = dir.join(&pair_group.id);
    if path.exists() {
        return Err(Error {
            message: String::from("Pair group to save already exists!"),
        });
    }
    write_pair_group(&data_access.root, pair_group)?;
    return Ok(());
}

impl UpdatePairGroupDataAccess for FileSystemDataAccess {
    async fn find_pair(&mut self, id: &str) -> Result<Option<Pair>, Error> {
        return find_pair(&self, id).await;
    }

    async fn delete_pair(&mut self, id: &str) -> Result<(), Error> {
        return delete_pair(&self, id).await;
    }

    async fn save_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        return save_pair(&self, pair).await;
    }

    async fn update_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        return update_pair(&self, pair).await;
    }

    async fn find_pair_group(&mut self, id: &str) -> Result<Option<PairGroup>, Error> {
        return find_pair_group(&self, id).await;
    }

    async fn update_pair_group(&mut self, pair_group: &PairGroup) -> Result<(), Error> {
        return update_pair_group(&self, pair_group).await;
    }
}

async fn find_pair(data_access: &FileSystemDataAccess, id: &str) -> Result<Option<Pair>, Error> {
    let entries = get_dir_entries(&data_access.root, PAIRS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(comparison_id) = file_name.to_str() {
            if comparison_id == id {
                let pair = read_pair(&data_access.root, id)?;
                return Ok(Some(pair));
            }
        }
    }
    return Ok(None);
}

impl DeletePairGroupDataAccess for FileSystemDataAccess {
    async fn find_pair_group(&mut self, id: &str) -> Result<Option<PairGroup>, Error> {
        return find_pair_group(&self, id).await;
    }

    async fn delete_pair(&mut self, id: &str) -> Result<(), Error> {
        return delete_pair(&self, id).await;
    }

    async fn delete_pair_group(&mut self, id: &str) -> Result<(), Error> {
        return delete_pair_group(&self, id).await;
    }
}

async fn find_pair_group(
    data_access: &FileSystemDataAccess,
    id: &str,
) -> Result<Option<PairGroup>, Error> {
    let entries = get_dir_entries(&data_access.root, PAIR_GROUPS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(comparison_id) = file_name.to_str() {
            if comparison_id == id {
                let pair_group = read_pair_group(&data_access.root, id)?;
                return Ok(Some(pair_group));
            }
        }
    }
    return Ok(None);
}

async fn delete_pair(data_access: &FileSystemDataAccess, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, PAIRS_DIR_NAME)?;
    let path = dir.join(id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Pair to delete does not exist!"),
        });
    }
    remove_pair(&data_access.root, id)?;
    return Ok(());
}

fn remove_pair(root: &Path, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(root, PAIRS_DIR_NAME)?;
    let path = dir.join(id);
    remove_object_file(&path)?;
    return Ok(());
}

fn remove_object_file(path: &Path) -> Result<(), Error> {
    remove_file(path).map_err(|e| Error {
        message: e.to_string(),
    })?;
    return Ok(());
}

async fn delete_pair_group(data_access: &FileSystemDataAccess, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, PAIR_GROUPS_DIR_NAME)?;
    let path = dir.join(id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Pair group to delete does not exist!"),
        });
    }
    remove_pair_group(&data_access.root, id)?;
    return Ok(());
}

// TODO: possibly refactor those `remove_` functions to a single one, since they're all doing the same
fn remove_pair_group(root: &Path, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(root, PAIR_GROUPS_DIR_NAME)?;
    let path = dir.join(id);
    remove_object_file(&path)?;
    return Ok(());
}

impl ViewPortfoliosDataAccess for FileSystemDataAccess {
    async fn fetch_tags(&mut self) -> Result<Vec<Tag>, Error> {
        return fetch_tags(&self).await;
    }

    async fn fetch_assets(&mut self) -> Result<Vec<Asset>, Error> {
        return fetch_assets(&self).await;
    }

    async fn update_asset(&mut self, asset: &Asset) -> Result<(), Error> {
        return update_asset(&self, asset).await;
    }
}

async fn fetch_tags(data_access: &FileSystemDataAccess) -> Result<Vec<Tag>, Error> {
    let mut tags: Vec<Tag> = vec![];
    let entries = get_dir_entries(&data_access.root, TAGS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(id) = file_name.to_str() {
            let tag = read_tag(&data_access.root, id)?;
            tags.push(tag);
        }
    }
    return Ok(tags);
}

fn read_tag(root: &Path, id: &str) -> Result<Tag, Error> {
    let dir = ensure_dir(root, TAGS_DIR_NAME)?;
    let path = dir.join(id);
    let fs_tag = create_object_from_file::<FileSystemTag>(&path)?;
    let mut tag = Tag {
        id: fs_tag.id.clone(),
        assets: vec![],
        name: fs_tag.name.clone(),
        created_at: fs_tag.created_at.clone(),
        updated_at: fs_tag.updated_at.clone(),
    };
    for asset_id in &fs_tag.assets {
        let asset = read_asset(root, asset_id)?;
        tag.assets.push(asset);
    }
    return Ok(tag);
}

fn read_asset(root: &Path, id: &str) -> Result<Asset, Error> {
    let dir = ensure_dir(root, ASSETS_DIR_NAME)?;
    let path = dir.join(id);
    let fs_asset = create_object_from_file::<FileSystemAsset>(&path)?;
    return Ok(Asset {
        id: fs_asset.id.clone(),
        coin: fs_asset.coin.clone(),
        quantity: fs_asset.quantity.clone(),
        usd_value: fs_asset.usd_value.clone(),
        created_at: fs_asset.created_at.clone(),
        updated_at: fs_asset.updated_at.clone(),
    });
}

async fn fetch_assets(data_access: &FileSystemDataAccess) -> Result<Vec<Asset>, Error> {
    let mut assets: Vec<Asset> = vec![];
    let entries = get_dir_entries(&data_access.root, ASSETS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(id) = file_name.to_str() {
            let asset = read_asset(&data_access.root, id)?;
            assets.push(asset);
        }
    }
    return Ok(assets);
}

impl StorePortfoliosDataAccess for FileSystemDataAccess {
    async fn find_tag(&mut self, id: &str) -> Result<Option<Tag>, Error> {
        return find_tag(&self, id).await;
    }

    async fn update_tag(&mut self, tag: &Tag) -> Result<(), Error> {
        return update_tag(&self, tag).await;
    }

    async fn save_asset(&mut self, asset: &Asset) -> Result<(), Error> {
        return save_asset(&self, asset).await;
    }
}

async fn find_tag(data_access: &FileSystemDataAccess, id: &str) -> Result<Option<Tag>, Error> {
    let entries = get_dir_entries(&data_access.root, TAGS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(comparison_id) = file_name.to_str() {
            if comparison_id == id {
                let tag = read_tag(&data_access.root, id)?;
                return Ok(Some(tag));
            }
        }
    }
    return Ok(None);
}

async fn update_tag(data_access: &FileSystemDataAccess, tag: &Tag) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, TAGS_DIR_NAME)?;
    let path = dir.join(&tag.id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Tag to update does not exist!"),
        });
    }
    write_tag(&data_access.root, tag)?;
    return Ok(());
}

fn write_tag(root: &Path, tag: &Tag) -> Result<(), Error> {
    let dir = ensure_dir(root, TAGS_DIR_NAME)?;
    let path = dir.join(&tag.id);
    write_object_file(
        &path,
        &FileSystemTag {
            id: tag.id.clone(),
            name: tag.name.clone(),
            assets: tag.assets.iter().map(|a| a.id.clone()).collect(),
            created_at: tag.created_at.clone(),
            updated_at: tag.updated_at.clone(),
        },
    )?;
    return Ok(());
}

fn write_asset(root: &Path, asset: &Asset) -> Result<(), Error> {
    let dir = ensure_dir(root, ASSETS_DIR_NAME)?;
    let path = dir.join(&asset.id);
    write_object_file(
        &path,
        &FileSystemAsset {
            id: asset.id.clone(),
            coin: asset.coin.clone(),
            quantity: asset.quantity.clone(),
            usd_value: asset.usd_value.clone(),
            created_at: asset.created_at.clone(),
            updated_at: asset.updated_at.clone(),
        },
    )?;
    return Ok(());
}

async fn save_asset(data_access: &FileSystemDataAccess, asset: &Asset) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, ASSETS_DIR_NAME)?;
    let path = dir.join(&asset.id);
    if path.exists() {
        return Err(Error {
            message: String::from("Asset to save already exists!"),
        });
    }
    write_asset(&data_access.root, asset)?;
    return Ok(());
}

impl SaveTagDataAccess for FileSystemDataAccess {
    async fn save_tag(&mut self, tag: &Tag) -> Result<(), Error> {
        return save_tag(&self, tag).await;
    }
}

async fn save_tag(data_access: &FileSystemDataAccess, tag: &Tag) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, TAGS_DIR_NAME)?;
    let path = dir.join(&tag.id);
    if path.exists() {
        return Err(Error {
            message: String::from("Tag to save already exists!"),
        });
    }
    write_tag(&data_access.root, tag)?;
    return Ok(());
}

impl DeleteTagDataAccess for FileSystemDataAccess {
    async fn delete_tag(&mut self, id: &str) -> Result<(), Error> {
        return delete_tag(&self, id).await;
    }
}

async fn delete_tag(data_access: &FileSystemDataAccess, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, TAGS_DIR_NAME)?;
    let path = dir.join(id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Tag to delete does not exist!"),
        });
    }
    remove_tag(&data_access.root, id)?;
    return Ok(());
}

fn remove_tag(root: &Path, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(root, TAGS_DIR_NAME)?;
    let path = dir.join(id);
    remove_object_file(&path)?;
    return Ok(());
}

impl UpdatePortfolioDataAccess for FileSystemDataAccess {
    async fn retrieve_tags_by_asset(&mut self, id: &str) -> Result<Vec<Tag>, Error> {
        return retrieve_tags_by_asset(&self, id).await;
    }

    async fn find_tag(&mut self, id: &str) -> Result<Option<Tag>, Error> {
        return find_tag(&self, id).await;
    }

    async fn update_tag(&mut self, tag: &Tag) -> Result<(), Error> {
        return update_tag(&self, tag).await;
    }

    async fn find_asset(&mut self, id: &str) -> Result<Option<Asset>, Error> {
        return find_asset(&self, id).await;
    }

    async fn update_asset(&mut self, asset: &Asset) -> Result<(), Error> {
        return update_asset(&self, asset).await;
    }
}

async fn retrieve_tags_by_asset(
    data_access: &FileSystemDataAccess,
    id: &str,
) -> Result<Vec<Tag>, Error> {
    let mut tags: Vec<Tag> = vec![];
    let entries = get_dir_entries(&data_access.root, TAGS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(tag_id) = file_name.to_str() {
            let tag = read_tag(&data_access.root, tag_id)?;
            if tag.assets.iter().any(|a| a.id == id) {
                tags.push(tag);
            }
        }
    }
    return Ok(tags);
}

async fn find_asset(data_access: &FileSystemDataAccess, id: &str) -> Result<Option<Asset>, Error> {
    let entries = get_dir_entries(&data_access.root, ASSETS_DIR_NAME)?;
    for entry in entries {
        let file_name = entry.file_name();
        if let Some(comparison_id) = file_name.to_str() {
            if comparison_id == id {
                let asset = read_asset(&data_access.root, id)?;
                return Ok(Some(asset));
            }
        }
    }
    return Ok(None);
}

async fn update_asset(data_access: &FileSystemDataAccess, asset: &Asset) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, ASSETS_DIR_NAME)?;
    let path = dir.join(&asset.id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Asset to update does not exist!"),
        });
    }
    write_asset(&data_access.root, asset)?;
    return Ok(());
}

impl DeleteAssetDataAccess for FileSystemDataAccess {
    async fn retrieve_tags_by_asset(&mut self, id: &str) -> Result<Vec<Tag>, Error> {
        return retrieve_tags_by_asset(&self, id).await;
    }

    async fn update_tag(&mut self, tag: &Tag) -> Result<(), Error> {
        return update_tag(&self, tag).await;
    }

    async fn delete_asset(&mut self, id: &str) -> Result<(), Error> {
        return delete_asset(&self, id).await;
    }
}

async fn delete_asset(data_access: &FileSystemDataAccess, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, ASSETS_DIR_NAME)?;
    let path = dir.join(id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Asset to delete does not exist!"),
        });
    }
    remove_asset(&data_access.root, id)?;
    return Ok(());
}

fn remove_asset(root: &Path, id: &str) -> Result<(), Error> {
    let dir = ensure_dir(root, ASSETS_DIR_NAME)?;
    let path = dir.join(id);
    remove_object_file(&path)?;
    return Ok(());
}

impl ViewWatchlistDataAccess for FileSystemDataAccess {
    async fn update_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        return update_pair(&self, pair).await;
    }

    async fn find_watchlist(&mut self) -> Result<Option<Watchlist>, Error> {
        return find_watchlist(&self).await;
    }

    async fn save_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error> {
        return save_watchlist(&self, watchlist).await;
    }

    async fn update_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error> {
        return update_watchlist(&self, watchlist).await;
    }
}

// NOTE: currently the business logic states that only one watchlist should exist
async fn find_watchlist(data_access: &FileSystemDataAccess) -> Result<Option<Watchlist>, Error> {
    let entries = get_dir_entries(&data_access.root, WATCHLISTS_DIR_NAME)?;
    if entries.len() == 0 {
        return Ok(None);
    }
    let first_entry = &entries[0];
    let file_name = first_entry.file_name();
    if let Some(id) = file_name.to_str() {
        let watchlist = read_watchlist(&data_access.root, id)?;
        return Ok(Some(watchlist));
    }
    return Ok(None);
}

fn read_watchlist(root: &Path, id: &str) -> Result<Watchlist, Error> {
    let dir = ensure_dir(root, WATCHLISTS_DIR_NAME)?;
    let path = dir.join(id);
    let fs_watchlist = create_object_from_file::<FileSystemWatchlist>(&path)?;
    let mut watchlist = Watchlist {
        id: fs_watchlist.id.clone(),
        pairs: vec![],
        created_at: fs_watchlist.created_at.clone(),
        updated_at: fs_watchlist.updated_at.clone(),
    };
    for pair_id in &fs_watchlist.pairs {
        let pair = read_pair(root, &pair_id)?;
        watchlist.pairs.push(pair);
    }
    return Ok(watchlist);
}

async fn save_watchlist(
    data_access: &FileSystemDataAccess,
    watchlist: &Watchlist,
) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, WATCHLISTS_DIR_NAME)?;
    let path = dir.join(&watchlist.id);
    if path.exists() {
        return Err(Error {
            message: String::from("Pair group to save already exists!"),
        });
    }
    write_watchlist(&data_access.root, watchlist)?;
    return Ok(());
}

async fn update_watchlist(
    data_access: &FileSystemDataAccess,
    watchlist: &Watchlist,
) -> Result<(), Error> {
    let dir = ensure_dir(&data_access.root, WATCHLISTS_DIR_NAME)?;
    let path = dir.join(&watchlist.id);
    if !path.exists() {
        return Err(Error {
            message: String::from("Pair group to update does not exist!"),
        });
    }
    write_watchlist(&data_access.root, watchlist)?;
    return Ok(());
}

fn write_watchlist(root: &Path, watchlist: &Watchlist) -> Result<(), Error> {
    let dir = ensure_dir(root, WATCHLISTS_DIR_NAME)?;
    let path = dir.join(&watchlist.id);
    write_object_file(
        &path,
        &FileSystemWatchlist {
            id: watchlist.id.clone(),
            pairs: watchlist.pairs.iter().map(|p| p.id.clone()).collect(),
            created_at: watchlist.created_at.clone(),
            updated_at: watchlist.updated_at.clone(),
        },
    )?;
    return Ok(());
}

impl StoreWatchlistCoinsDataAccess for FileSystemDataAccess {
    async fn save_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        return save_pair(&self, pair).await;
    }

    async fn get_watchlist(&mut self) -> Result<Watchlist, Error> {
        return get_watchlist(&self).await;
    }

    async fn update_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error> {
        return update_watchlist(&self, watchlist).await;
    }
}

async fn get_watchlist(data_access: &FileSystemDataAccess) -> Result<Watchlist, Error> {
    if let Some(watchlist) = find_watchlist(data_access).await? {
        return Ok(watchlist);
    } else {
        return Err(Error {
            message: String::from("Watchlist not found!"),
        });
    };
}

impl DeleteWatchlistPairDataAccess for FileSystemDataAccess {
    async fn delete_pair(&mut self, id: &str) -> Result<(), Error> {
        return delete_pair(&self, id).await;
    }

    async fn get_watchlist(&mut self) -> Result<Watchlist, Error> {
        return get_watchlist(&self).await;
    }

    async fn update_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error> {
        return update_watchlist(&self, watchlist).await;
    }
}

#[cfg(test)]
mod tests {
    /*
       TODO:
           - Unit tests for save pair group
           - Unit tests for delete pair group
    */
    use crate::entities::pair::Pair;

    use super::*;
    use chrono::Utc;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_fetch_pair_groups() {
        /*
            Unit test expectations:

            - The fetched number of pair groups equals the number of written pair groups.
            - Each fetched pair group matches the corresponding example pair group.
        */
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        let example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 1.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p2".to_string(),
                value: 2.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p3".to_string(),
                value: 3.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        ];

        let example_pair_groups = vec![
            PairGroup {
                id: "pg1".to_string(),
                is_pinned: true,
                multiplier: 1.0,
                pairs: vec![example_pairs[0].clone(), example_pairs[1].clone()],
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            PairGroup {
                id: "pg2".to_string(),
                is_pinned: false,
                multiplier: 1.0,
                pairs: vec![example_pairs[2].clone()],
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        ];

        for example_pair_group in &example_pair_groups {
            write_pair_group(&root, example_pair_group).unwrap();
        }

        let data_access: FileSystemDataAccess = FileSystemDataAccess {
            root: root.to_path_buf(),
        };
        let pair_groups = fetch_pair_groups(&data_access).await.unwrap();

        assert_eq!(pair_groups.len(), 2);
        assert_eq!(pair_groups[0], example_pair_groups[0]);
        assert_eq!(pair_groups[1], example_pair_groups[1]);

        std::fs::remove_dir_all(root).expect("Failed to clear test temp directory");
    }

    #[tokio::test]
    async fn test_update_pair_group() {
        /*
            Unit test expectations:

            - The previously written pair group should be replaced by the updated pair group.
        */
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        let example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 1.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p2".to_string(),
                value: 2.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p3".to_string(),
                value: 3.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        ];

        let original_pair_group = PairGroup {
            id: "pg1".to_string(),
            is_pinned: false,
            multiplier: 1.0,
            pairs: vec![example_pairs[0].clone(), example_pairs[1].clone()],
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        write_pair_group(&root, &original_pair_group).unwrap();

        let updated_pair_group = PairGroup {
            id: "pg1".to_string(),
            is_pinned: true,
            multiplier: 1.0,
            pairs: vec![
                example_pairs[0].clone(),
                example_pairs[1].clone(),
                example_pairs[2].clone(),
            ],
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let data_access: FileSystemDataAccess = FileSystemDataAccess {
            root: root.to_path_buf(),
        };
        update_pair_group(&data_access, &updated_pair_group)
            .await
            .unwrap();

        let stored_pair_group = read_pair_group(root, "pg1").unwrap();
        assert_eq!(stored_pair_group, updated_pair_group);

        std::fs::remove_dir_all(root).expect("Failed to clear test temp directory");
    }
}
