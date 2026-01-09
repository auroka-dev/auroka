use crate::Page;
use crate::PageConfig;
use std::future::Future;

#[doc(hidden)]
pub async fn with_page_internal<F, Fut>(
  config: Option<PageConfig>,
  path: &str,
  test_fn: F,
) -> anyhow::Result<()>
where
  F: FnOnce(Page) -> Fut,
  Fut: Future<Output = anyhow::Result<()>>,
{
  if !path.starts_with("http") {
    return Err(anyhow::anyhow!(
      "URL must start with 'http' or 'https'. Relative paths matching implicit base URL are no longer supported."
    ));
  }

  let page = if let Some(config) = config {
    let page = Page::launch(config).await?;
    page.navigate(path).await?;
    page
  } else {
    Page::goto(path).await?
  };

  let result = test_fn(page.clone()).await;
  let close_result = page.close().await;
  result.and(close_result)
}

#[macro_export]
macro_rules! with_page {
  // --- Internal Parsing Rules ---
  // These rules consume tokens one by one (or in groups) to build the configuration.

  // 1. :Mobile - Enable mobile mode
  (@parse $config:expr, :Mobile $($rest:tt)*) => {
    $crate::with_page!(@parse $config.mobile(true), $($rest)*)
  };

  // 2. :Landscape - Layout orientation
  (@parse $config:expr, :Landscape $($rest:tt)*) => {
    $crate::with_page!(@parse $config.landscape(true), $($rest)*)
  };

  // 3. :GeoLocation(lat, long)
  (@parse $config:expr, :GeoLocation($lat:expr, $long:expr) $($rest:tt)*) => {
    $crate::with_page!(@parse $config.geolocation($lat, $long), $($rest)*)
  };

  // 4. :Permissions([vec])
  (@parse $config:expr, :Permissions($perms:expr) $($rest:tt)*) => {
    $crate::with_page!(@parse $config.permissions($perms), $($rest)*)
  };

  // 5. Preset Resolutions (HD, FHD, 4K, 5K)
  (@parse $config:expr, :HD $($rest:tt)*) => {
    $crate::with_page!(@parse $config.viewport_preset($crate::Viewport::HD), $($rest)*)
  };

  (@parse $config:expr, :FHD $($rest:tt)*) => {
    $crate::with_page!(@parse $config.viewport_preset($crate::Viewport::FHD), $($rest)*)
  };

  (@parse $config:expr, :4K $($rest:tt)*) => {
    $crate::with_page!(@parse $config.viewport_preset($crate::Viewport::UHD4K), $($rest)*)
  };

  (@parse $config:expr, :5K $($rest:tt)*) => {
    $crate::with_page!(@parse $config.viewport_preset($crate::Viewport::UHD5K), $($rest)*)
  };

  // 6. Custom Resolution :1280x720
  // Note: Rust tokenizes `1280x720` as a single literal if it starts with a digit?
  // Actually, Rust lexer splits `1280x720` into `1280` (integer literal) and `x720` (identifier)
  // or `1280` `x` `720` depending on spacing.
  // The user wants `:1280x720`.
  // If the user writes `:1280x720`, it might tokenise as colon, then...
  // Wait, `1280x720` is not a standard literal.
  // Standard Rust macro matching for `:$w x $h` works if there are spaces `:1280 x 720`.
  // To support `:1280x720` (no spaces), we might need to rely on how Rust sees it.
  // Often it sees `1280` `x720` if `x` is identifier start.
  // Let's stick to the space-separated version for custom reliable parsing or accept both.

  // Custom resolution with spaces :1280 x 720
  (@parse $config:expr, :$w:literal x $h:literal $($rest:tt)*) => {
    $crate::with_page!(@parse $config.viewport($w, $h), $($rest)*)
  };

  // 7. 'using' config override
  // usage: with_page!(using my_config, ...)
  (@parse $config:expr, using $user_config:expr, $($rest:tt)*) => {
    $crate::with_page!(@parse $user_config, $($rest)*)
  };

  // 8. Generic Browser Variant (must come after specific keywords)
  // usage: :Chrome :Firefox
  (@parse $config:expr, :$browser:ident $($rest:tt)*) => {
    $crate::with_page!(@parse $config.browser($crate::Browser::$browser), $($rest)*)
  };

  // Handle commas optionally?
  (@parse $config:expr, , $($rest:tt)*) => {
    $crate::with_page!(@parse $config, $($rest)*)
  };

  // --- Termination Rules ---
  // These match the final arguments (Base + Path, or just Path) and the Closure.

  // Case A: Base URL + Path
  (@parse $config:expr, $base:expr, $path:expr, |$page:ident| $body:block) => {
    $crate::with_page!(@parse $config, &format!("{}{}", $base, $path), |$page| $body)
  };

  // Case B: Single URL
  (@parse $config:expr, $url:expr, |$page:ident| $body:block) => {
    $crate::with_page_internal(
      Some($config.into()),
      $url,
      |$page| async move {
        $body
        Ok(())
      }
    )
    .await
  };

  // --- Public Entry Point ---
  // Transforms the input into the recursive parser format with a default config.
  ($($args:tt)*) => {
    $crate::with_page!(@parse $crate::PageConfig::new(), $($args)*)
  };
}
