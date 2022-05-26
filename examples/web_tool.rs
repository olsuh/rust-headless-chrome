use std::{fs, thread, }; //vec

use std::time::{Duration, Instant};
use std::ffi::OsStr;

use anyhow::Result;

use headless_chrome::types::Bounds;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptions};

//use (crate)::logging;
//mod logging;

fn main() -> Result<()> {

    let headless = true; let screen_shot = false; let incognito = true;
    //let headless = false;
    let url = "https://majortrade.pro/".to_string();
    //let url = "https://intoli.com/blog/not-possible-to-block-chrome-headless/chrome-headless-test.html";
    let wait_context_selector = format!(r#"[href="{}"]"#, url);
    let timeout_sec = 600;

    let mut handles = vec![];

    for ind in 1..=7 {
        let url = url.clone();
        let wait_context_selector = wait_context_selector.clone();
        let handle = thread::spawn( move || {
            
            let str = web_tool(ind, &url, headless, incognito, &wait_context_selector, timeout_sec, screen_shot);
            str
        });
        handles.push(handle);
    }

    for handle in handles {
        let ret = handle.join().unwrap();
        println!("{:?}", ret);
    }

    Ok(())
}

fn web_tool(ind: usize, url: &str, headless: bool, incognito: bool, wait_context_selector: &str, timeout_sec: u64, screen_shot: bool) -> Result<String> {

    println!("web_tool {} to {}...", ind, url);

    let options = LaunchOptions {
            headless: headless, 
            window_size: Some((1200,800)),
            args: vec![ OsStr::new("--incognito") ],
            idle_browser_timeout: std::time::Duration::from_secs(600),
            ..Default::default()
    };

    //logging::enable_logging();
    let browser = Browser::new(options)?;
    //let num_tabs = browser.get_tabs().lock().unwrap().len();
    //dbg!(num_tabs);

    let tab = if !incognito {
         browser.wait_for_initial_tab()?
    } else {
        let incognito_context = browser.new_context()?;
        incognito_context.new_tab()?
    };

    //let num_tabs = browser.get_tabs().lock().unwrap().len();
    //dbg!(num_tabs);

    let now = Instant::now();
    //tab.set_bounds(Bounds::Fullscreen)?;
    //tab.enable_stealth();
    tab.bypass_wedriver()?;
    tab.bypass_chrome()?;
    tab.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36", None, None)?;

    //maybe:
    //tab.bypass_permissions()?;
    // bad:
    //tab.bypass_plugins()?;
    //tab.bypass_webgl_vendor()?;

    tab
        .navigate_to(url)?
        .wait_until_navigated()?;

    //let title = tab.get_title()?;
    //dbg!(title);

    let _el = tab.wait_for_element_with_custom_timeout(wait_context_selector, Duration::from_secs(timeout_sec)).ok();
    //     .get_inner_text()?;

    let title = tab.get_title()?;
    let elapsed = now.elapsed().as_secs_f32();
    println!("wt {} titele: {} elapsed: {}", ind, title, elapsed);

    let mut cookies_string = "".to_string();
    let cookies = tab.get_cookies().unwrap_or_default();

    //let result = tab.call_method(Network::GetCookies { urls: None }).unwrap();
    //let mut result_string = format!("{:?}", result);

    for ( i, cookie) in cookies.iter().enumerate() {
        
        let s = format!("{:?}", cookie);
        //println!("{}", s);
        if i > 0 {
            cookies_string.push_str("\n");
        }
        cookies_string.push_str(&s);
    }

    if headless && screen_shot {
        let png_data = tab
            .capture_screenshot(CaptureScreenshotFormatOption::Png,Some(75), None, true)?;
        fs::write("screen_001.png", &png_data)?;
        println!("wt {} Screenshots successfully created.", ind);
    }

    
    Ok(cookies_string)
}
