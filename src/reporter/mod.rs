use std::fs;
use std::error::Error;
use chrono::{FixedOffset, Utc};
use crate::{file, result};

pub struct Reporter {
    pub file_name: String,
}

impl Reporter {
    fn new(file_name: String) -> Self {
        Self { file_name }
    }

    pub fn generate(&self, results: Vec<result::Result>) -> Result<(), Box<dyn Error>> {
        println!("Generate report to html file");
        println!("Generate file {}", self.file_name);

        let html_tmpl = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Test Runner - Report</title>
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap" rel="stylesheet">
                <style>
                    html, body { font-family: 'Roboto', sans-serif; } body { margin: 16px 64px; } th { background-color: #f7f7f7; } a { color: white; } th, td { padding: 0.5rem; } .success { color: #008000; } .failed { color: #d40501; } .medium { font-weight: 500; } .hide { display: none; } .show { display: block; } table { table-layout: fixed; width: 100%; display: table; } pre { overflow: scroll; } h4 { padding: 4px 8px; } .coverage { font-size: small; font-weight: normal; } tr.passed { background: #dfd; } tr.skipped { background: #eee; } tr.failed { background: #fdd; } .center { margin-left: auto; margin-right: auto; } .badge { background: #888; color: white; border-radius: 4px; padding: 2px 8px; float: right; } .passed .badge { background: #5c5; ; } .skipped .badge {} .failed .badge { background-color: #d66; } .expando { padding: 8px; display: none; } .content { padding: 16px 0; border-bottom: thin #eeeeee solid; color: #444; white-space: pre; } .duration { font-size: x-small; } div:target .expando { display: block }
                </style>
            </head>
            <body>
                <h1>UI Test Report</h1>
                <p>Generated: {1}</p>
                {2}
            </body>
            </html>
            "#;

        let passed = results.iter().filter(|r| r.status == "PASSED").count();
        let failed = results.iter().filter(|r| r.status == "FAILED").count();
        let times: i64 = results.iter().map(|r| r.get_time_usage()).sum();
        let time_minute = times as f32 / 60.0;

        let mut html_content = format!(
            r#"
            <p>Tests: {}</p><p>Pass: {}</p><p>Failed: {}</p><p>Times: {:.2} m</p>
            <table style="width: 100%;">
            "#,
            results.len(),
            passed,
            failed,
            time_minute
        );

        for r in &results {
            let status = if !r.error.is_empty() {
                format!(
                    r#"<span class="badge"><a href="?error={}">{}</a></span>"#,
                    &r.test,
                    &r.status
                )
            } else {
                format!(r#"<span class="badge">{}</span>"#, r.status)
            };

            let status_class = r.status.to_lowercase();
            html_content += &format!(
                r#"
                <tr class="{}">
                    <td>{}</td><td>{} s</td><td>{}</td>
                </tr>
                <tr id="{}" class="hide">
                    <td colspan="3"><pre>{}</pre></td>
                </tr>
                "#,
                status_class,
                &r.test,
                r.get_time_usage(),
                status,
                &r.test,
                &r.error
            );
        }
        html_content += "</table>";
        html_content += r#"
        <script>
            const params = new Proxy(new URLSearchParams(window.location.search), { get: (searchParams, prop) => searchParams.get(prop), });
            let error = params['error'];
            if (!!error) {
                var element = document.getElementById(error);
                element.classList.remove('hide');
            }
        </script>
        "#;

        let offset = FixedOffset::east_opt(7 * 3600).unwrap();
        let current_date = Utc::now().with_timezone(&offset).format("%Y-%m-%d %H:%M:%S").to_string();

        let html5 = html_tmpl.replace("{1}", &current_date).replace("{2}", &html_content);

        // Create file
        file::check_or_create_is_not_exist_dir(&self.file_name);
        fs::write(&self.file_name, html5)?;

        println!("Test result");
        println!("ALL: {}", results.len());
        println!("PASSED: {}", passed);
        println!("FAILED: {}", failed);
        println!("TIMES: {:.2} m", time_minute);

        Ok(())
    }
}