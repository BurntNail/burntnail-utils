use crate::{
    error_ext::{ErrorExt, ToAnyhowErr, ToAnyhowNotErr},
    time_based_structs::scoped_timers::ScopedTimer,
};
use find_folder::Search::ParentsThenKids;
use piston_window::{
    Filter, Flip, G2dTexture, G2dTextureContext, PistonWindow, Texture, TextureSettings,
};
use std::{collections::HashMap, path::PathBuf, result::Result as SResult};

//copied from piston_gfx_texture
/// Creates a texture from path.
pub fn from_path<F, C, P>(
    context: &mut TextureContext<F, R, C>,
    path: P,
    flip: Flip,
    settings: &TextureSettings,
) -> Result<Self, Error>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    P: AsRef<Path>,
{
    let img = image::open(path).map_err(|e| e.to_string())?;

    let img = match img {
        DynamicImage::ImageRgba8(img) => img,
        img => img.to_rgba8(),
    };

    let img = match flip {
        Flip::Vertical => image::imageops::flip_vertical(&img),
        Flip::Horizontal => image::imageops::flip_horizontal(&img),
        Flip::Both => {
            let img = image::imageops::flip_vertical(&img);
            image::imageops::flip_horizontal(&img)
        }
        Flip::None => img,
    };

    Texture::from_image(context, &img, settings)
}

#[cfg(feature = "anyhow")]
use anyhow::{anyhow, Context, Result};

///Struct to hold a cache of [`G2dTexture`]s
pub struct Cacher {
    ///Base path for the assets
    base_path: PathBuf,
    ///HashMap of paths to textures
    assets: HashMap<String, G2dTexture>,
    ///Context for textures from window
    tc: G2dTextureContext,
}

impl Cacher {
    ///Function to create a new empty cache.
    ///
    /// # Errors
    /// Can fail if it can't find the assets folder
    fn base_new(win: &mut PistonWindow) -> SResult<Self, find_folder::Error> {
        let path = ParentsThenKids(2, 2)
            .for_folder("assets")
            .context("Finding the assets folder")?;
        Ok(Self {
            base_path: path,
            assets: HashMap::new(),
            tc: win.create_texture_context(),
        })
    }

    fn base_get(&mut self, p: &str) -> SResult<Option<&G2dTexture>, String> {
        self.base_insert(p).map(|_| self.assets.get(p))
    }

    fn base_insert(&mut self, p: &str) -> Result<(), String> {
        if self.assets.contains_key(p) {
            return Ok(());
        }

        cfg_if! {
            if #[cfg(feature = "tracing")] {
                tracing::trace!("Inserting {p}");
            } else {
                println!("Inserting {p}");
            }
        }
        let _st = ScopedTimer::new(format!("Geting {p}"));

        let path = self.base_path.join(p);
        let ts = TextureSettings::new().filter(Filter::Nearest);

        match Texture::from_path(&mut self.tc, path, Flip::None, &ts) {
            Ok(tex) => {
                self.assets.insert(p.to_string(), tex);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(feature = "anyhow")]
impl Cacher {
    ///Function to create a new empty cache.
    ///
    /// # Errors
    /// Can fail if it can't find the assets folder
    pub fn new(win: &mut PistonWindow) -> Result<Self> {
        Cacher::base_new(win).ae()
    }

    ///Gets a [`G2dTexture`] from the cache. Returns [`None`] if there is no asset with that path.
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn get(&mut self, p: &str) -> Result<&G2dTexture> {
        match self.base_get(p) {
            Ok(opt) => Ok(opt
                .ae()
                .context("getting asset that exists")
                .unwrap_log_error()),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    ///Inserts a new asset into the cache from the path given - should just be like `'icon.png'`, as all files should be in the `'assets/'` folder
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    fn insert(&mut self, p: &str) -> Result<()> {
        self.base_insert(p).map_err(|s| anyhow::Error::new(s))
    }
}

#[cfg(not(feature = "anyhow"))]
impl Cacher {
    ///Function to create a new empty cache.
    ///
    /// # Errors
    /// Can fail if it can't find the assets folder
    pub fn new(win: &mut PistonWindow) -> SResult<Self, find_folder::Error> {
        Cacher::base_new(win)
    }

    ///Gets a [`G2dTexture`] from the cache. Returns [`None`] if there is no asset with that path.
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    pub fn get(&mut self, p: &str) -> SResult<&G2dTexture> {
        self.insert(p).map(|_| {
            self.assets
                .get(p)
                .ae()
                .context("getting asset that exists")
                .unwrap_log_error()
        })
    }

    ///Inserts a new asset into the cache from the path given - should just be like `'icon.png'`, as all files should be in the `'assets/'` folder
    ///
    /// # Errors
    /// - Unable to find the texture using [`Texture::from_path`]
    fn insert(&mut self, p: &str) -> Result<()> {
        if self.assets.contains_key(p) {
            return Ok(());
        }

        cfg_if! {
            if #[cfg(feature = "tracing")] {
                tracing::trace!("Inserting {p}");
            } else {
                println!("Inserting {p}");
            }
        }
        let _st = ScopedTimer::new(format!("Geting {p}"));

        let path = self.base_path.join(p);
        let ts = TextureSettings::new().filter(Filter::Nearest);

        match Texture::from_path(&mut self.tc, path, Flip::None, &ts) {
            Ok(tex) => {
                self.assets.insert(p.to_string(), tex);
                Ok(())
            }
            Err(e) => Err(anyhow!("Unable to find texture: {e}")),
        }
    }
}
