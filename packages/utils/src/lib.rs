pub mod env;
pub mod file;

#[derive(Debug, Clone)]
pub enum Origin {
    Github,
    Gitee,
}
impl Origin {
    pub fn get_pkg_url(&self) -> String {
        match self {
            Origin::Github => String::from("https://github.com/ngd-b"),
            Origin::Gitee => String::from("https://gitee.com/hboot"),
        }
    }
}

// 固定版本信息，
const VERSION: &str = "latest";

/// 根据系统获取rsup、rsup-web下载的地址
///
/// # Arguments:
/// * os 操作系统
/// * origin 下载源
///
/// # Returns
/// * rsup 下载地址
/// * rsup-web 下载地址
///
pub fn get_pkg_url(origin: Option<Origin>) -> (String, String) {
    let os = std::env::consts::OS;

    let origin = origin.unwrap_or(Origin::Github);
    // 下载地址
    let mut url = format!(
        "{}/rsup/releases/download/{}",
        origin.get_pkg_url(),
        VERSION
    );
    let mut web_url = format!(
        "{}/rsup-web/releases/download/{}",
        origin.get_pkg_url(),
        VERSION
    );

    // 根据系统不同下载不同的包，
    // 后缀名不一样
    let file_suffix = match os {
        "windows" => "windows-latest",
        "macos" => "macos-latest",
        _ => "ubuntu-latest",
    };
    url = format!("{}/rsup-{}.tar.gz", url, file_suffix);
    web_url = format!("{}/rsup-web.tar.gz", web_url);

    (url, web_url)
}
