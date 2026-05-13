use html_escape::encode_safe;
use regex::Regex;

/// 构建高亮匹配结果文本
pub fn build(re: &Regex, content: &str) -> String {
    let mut result = String::new();

    let mut last_end = 0;

    for m in re.find_iter(content) {
        // 未匹配部分
        result.push_str(&encode_safe(&content[last_end..m.start()]));

        // 匹配部分
        result.push_str("<code>");

        result.push_str(&encode_safe(m.as_str()));

        result.push_str("</code>");

        last_end = m.end();
    }

    // 尾部
    result.push_str(&encode_safe(&content[last_end..]));

    result
}
