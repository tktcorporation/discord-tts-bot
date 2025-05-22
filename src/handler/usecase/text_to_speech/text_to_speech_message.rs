use super::speech_options::SpeechOptions;
use super::SpeechMessage;
pub use crate::model::Message;
use regex::Regex;
use regex::RegexBuilder;

impl Message {
    /// Discordのメッセージを読み上げ用のメッセージに変換します。
    ///
    /// 主な処理は以下の通りです。
    /// - URLを読み上げ対象から除外します。
    /// - Discord特有の文字列（メンション、絵文字など）を適切な形に変換します。
    /// - メッセージが50文字を超える場合は、50文字にトリムし、「うぬんかんぬん」を末尾に追加します。
    pub fn to_speech_message(&self, _options: SpeechOptions) -> SpeechMessage {
        let content = self.get_content();

        // 最初にDiscord特有表現（スポイラーを含む）を処理
        let converted_content = convert_discord_string(&content);

        // 次にURLをスキップ
        let str_after_url_skip = converted_content
            .split_whitespace()
            .filter(|word| !word.starts_with("http"))
            .collect::<Vec<&str>>()
            .join(" ");

        // 最後に50文字制限
        // 50文字を超えた場合は、トリムして語尾に「うぬんかんぬん」を追加
        let message = if str_after_url_skip.chars().count() > 50 {
            let trimmed = str_after_url_skip.chars().take(50).collect::<String>();
            format!("{trimmed}うぬんかんぬん")
        } else {
            str_after_url_skip
        };

        SpeechMessage { value: message }
    }
}

/// Discord特有の文字列の種類を表す列挙型。
#[derive(Debug)]
enum DiscordStringType {
    Channel,
    Role,
    Emoji,
    Animoji,
    Mention,
    Spoiler,
}
impl DiscordStringType {
    /// 各Discord文字列の種類に対応する正規表現を返します。
    fn to_regex(&self) -> Regex {
        match self {
            DiscordStringType::Channel => Regex::new(r"<#[0-9]+?>").unwrap(),
            DiscordStringType::Role => Regex::new(r"<@&[0-9]+?>").unwrap(),
            DiscordStringType::Emoji => Regex::new(r"<:(.+?):[0-9]+?>").unwrap(),
            DiscordStringType::Animoji => Regex::new(r"<a:(.+?):[0-9]+?>").unwrap(),
            DiscordStringType::Mention => Regex::new(r"<@[0-9]+?>").unwrap(),
            DiscordStringType::Spoiler => RegexBuilder::new(r"\|\|.*?\|\|")
                .dot_matches_new_line(true)
                .build()
                .unwrap(),
        }
    }
    /// 各Discord文字列の種類に対応する変換方法を返します。
    fn to_convert_type(&self) -> ConvertType {
        match self {
            DiscordStringType::Channel => ConvertType::Empty,
            DiscordStringType::Role => ConvertType::Empty,
            DiscordStringType::Emoji => ConvertType::MatchString,
            DiscordStringType::Animoji => ConvertType::MatchString,
            DiscordStringType::Mention => ConvertType::Empty,
            DiscordStringType::Spoiler => ConvertType::Empty,
        }
    }
    /// 文字列がどのDiscord文字列の種類に一致するかを判定します。
    fn from_str(s: &str) -> Option<DiscordStringType> {
        let type_ = DiscordStringType::Channel;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Role;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Emoji;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Animoji;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Mention;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Spoiler;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        None
    }
}

/// Discord特有の文字列の変換方法を表す列挙型。
enum ConvertType {
    Empty,
    MatchString,
}
impl ConvertType {
    /// 指定された正規表現と変換方法に基づいて文字列を変換します。
    ///
    /// `Empty`の場合は、マッチした部分を空文字列に置換します。
    /// `MatchString`の場合は、正規表現の最初のキャプチャグループの内容に置換します。
    fn convert(&self, regex: &Regex, str: &str) -> String {
        match self {
            ConvertType::Empty => regex.replace_all(str, "").to_string(),
            ConvertType::MatchString => {
                let replacement = |cap: &regex::Captures| cap[1].to_string();
                regex.replace_all(str, replacement).to_string()
            }
        }
    }
}

/// Discord特유の文字列（メンション、絵文字、チャンネルリンクなど）を
/// 読み上げに適した形に再帰的に変換します。
/// スポイラー（||...||）は最優先で処理され、その中身は読み上げられません。
fn convert_discord_string(str: &str) -> String {
    // まずスポイラーを処理する
    let spoiler_regex = DiscordStringType::Spoiler.to_regex();
    let after_spoiler_conversion = spoiler_regex.replace_all(str, "").to_string();

    // スポイラー処理後の文字列に対して、他のDiscord特有文字列の処理を行う
    let (re, convert_type) =
        if let Some(type_) = DiscordStringType::from_str(&after_spoiler_conversion) {
            // スポイラーは既に処理済みなので、ここではスキップする
            if matches!(type_, DiscordStringType::Spoiler) {
                return after_spoiler_conversion;
            }
            (type_.to_regex(), type_.to_convert_type())
        } else {
            return after_spoiler_conversion; // 他に変換対象がなければそのまま返す
        };

    // スポイラー以外の変換処理
    // ここで再帰呼び出しを行うのは、一つの文字列に複数の異なるタイプの要素が含まれる場合に対応するため
    // 例: "<@mention> <:emoji:>"
    convert_discord_string(&convert_type.convert(&re, &after_spoiler_conversion))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::model::channel::Message as SerenityMessage;

    #[cfg(test)]
    mod convert_discord_string_test {
        use super::*;

        #[test]
        fn test_remove_mention_string() {
            let str = "aaa<@8379454856049>eeee";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeee", result);
        }
        #[test]
        fn test_remove_double_mention_string() {
            let str = "aaa<@8379454856049>eeee<@8379454856049>uuu";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeeeuuu", result);
        }

        #[test]
        fn test_remove_role_string() {
            let str = "aaa<@&8379454856049>eeee";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeee", result);
        }
        #[test]
        fn test_remove_double_role_string() {
            let str = "aaa<@&8379454856049>eeee<@&8379454856049>uuu";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeeeuuu", result);
        }

        #[test]
        fn test_remove_emoji_string() {
            let str = "<:butter:872873394570424340>";
            let result = convert_discord_string(str);
            assert_eq!("butter", result);
        }

        #[test]
        fn test_remove_animoji_string() {
            let str = "<a:sanma:872873394570424340>";
            let result = convert_discord_string(str);
            assert_eq!("sanma", result);
        }

        #[test]
        fn test_remove_double_emoji_string() {
            let content = "<:butter:872873394570424340>さんま<:sanma:872873394570424340>";
            let result = convert_discord_string(content);
            assert_eq!("butterさんまsanma", result);
        }

        #[test]
        fn test_replace_channel_string() {
            let str = "aaa<#795680552845443113>rrr";
            let result = convert_discord_string(str);
            assert_eq!("aaarrr", result);
        }
        #[test]
        fn test_replace_double_channel_string() {
            let str = "aaa<#795680552845443113>rrr<#795680552845443113>sss";
            let result = convert_discord_string(str);
            assert_eq!("aaarrrsss", result);
        }

        #[test]
        fn test_spoiler_string() {
            let str = "これは||ネタバレ内容||です";
            let result = convert_discord_string(str);
            assert_eq!("これはです", result);
        }

        #[test]
        fn test_multiple_spoiler_string() {
            let str = "これは||ネタバレ1||と||ネタバレ2||です";
            let result = convert_discord_string(str);
            assert_eq!("これはとです", result);
        }

        #[test]
        fn test_spoiler_with_url_and_text() {
            let str = "これは||ネタバレ内容 https://example.com||です";
            let result = convert_discord_string(str);
            assert_eq!("これはです", result);
        }

        #[test]
        fn test_spoiler_with_url_only() {
            let str = "これは||https://example.com||です";
            let result = convert_discord_string(str);
            assert_eq!("これはです", result);
        }

        #[test]
        fn test_spoiler_with_url_only_with_new_line() {
            let str = "||改行の次にURLが含まれてる場合、URL以外の部分を読み上げてしまうあわあわあわあわあわあわあわあわあわあわあわあわ。
https://x.com/tktcorporation/status/1925197887820140825||";
            let result = convert_discord_string(str);
            assert_eq!("", result);
        }

        #[test]
        fn test_spoiler_with_text_and_url() {
            let str = "これは||ネタバレ内容 https://example.com||です";
            let result = convert_discord_string(str);
            assert_eq!("これはです", result);
        }

        #[test]
        fn test_spoiler_at_beginning_with_url_and_text() {
            let str = "||https://example.com ネタバレ内容||です";
            let result = convert_discord_string(str);
            assert_eq!("です", result);
        }

        #[test]
        fn test_spoiler_at_end_with_url_and_text() {
            let str = "これは||https://example.com ネタバレ内容||";
            let result = convert_discord_string(str);
            assert_eq!("これは", result);
        }
    }

    #[cfg(test)]
    mod to_speech_message_tests {
        use super::*;
        #[test]
        fn test_message() {
            let message = message_factory("https://example.com");
            assert_eq!(
                "",
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_not_ssl() {
            let message = message_factory("http://example.com");
            assert_eq!(
                "",
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_url_in_text() {
            let message = message_factory("おはよう https://example.com こんにちは");
            assert_eq!(
                "おはよう こんにちは",
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_mix() {
            let message = message_factory("<@8379454856049>おはよう<:sanma:872873394570424340>こんにちは<#795680552845443113>でも<@&8379454856049>これは<@&8379454856049><:butter:872873394570424340>です");
            assert_eq!(
                "おはようsanmaこんにちはでもこれはbutterです",
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_trimmed_message() {
            let long_text = "あ".repeat(100);
            let message = message_factory(format!("{long_text}いいいい").as_str());
            assert_eq!(
                format!("ああああああああああああああああああああああああああああああああああああああああああああああああああうぬんかんぬん"),
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_spoiler_with_url_and_long_text_exceeding_limit() {
            let long_spoiler_text = "あ".repeat(60);
            let message_content = format!("||{long_spoiler_text} https://example.com||");
            let message = message_factory(&message_content);
            assert_eq!(
                "",
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value,
                "スポイラー内のURLと長い文字列は除去され、空文字列になるべき"
            );
        }

        #[test]
        fn test_message_with_spoiler_exceeding_limit_after_spoiler_removal() {
            let text_around_spoiler = "あ".repeat(60);
            let message_content =
                format!("先頭のテキスト||ネタバレ||{text_around_spoiler}末尾のテキスト");
            let message = message_factory(&message_content);
            // let expected_trimmed_text = format!("先頭のテキスト{}末尾のテキスト", "あ".repeat(34)); // "先頭のテキスト" (7) + "あ"*34 + "末尾のテキスト" (7) = 48. "あ"*35だと49. "あ"*36だと50.
            // "先頭のテキストあ...あ末尾のテキスト" (7 + 36 + 7 = 50) うぬんかんぬん となるように調整
            // let expected_message = format!(
            //     "{}うぬんかんぬん",
            //     "先頭のテキストあ"
            //         .repeat(50)
            //         .chars()
            //         .take(50)
            //         .collect::<String>()
            // );

            // 正確な期待値を計算
            // スポイラー除去後: "先頭のテキスト" + text_around_spoiler + "末尾のテキスト"
            // "先頭のテキスト" (7文字)
            // text_around_spoiler (60文字)
            // "末尾のテキスト" (7文字)
            // 合計: 7 + 60 + 7 = 74文字
            // 期待値: "先頭のテキスト" + "あ"*36 + "うぬんかんぬん"
            // "先頭のテキストああああああああああああああああああああああああああああああああああああうぬんかんぬん"
            let mut expected = String::new();
            expected.push_str("先頭のテキスト");
            expected.push_str(&("あ".repeat(60)));
            expected.push_str("末尾のテキスト");
            let trimmed_expected = expected.chars().take(50).collect::<String>();

            assert_eq!(
                format!("{trimmed_expected}うぬんかんぬん"),
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value,
                "スポイラー除去後、残りのメッセージが50文字にトリムされ、「うぬんかんぬん」が付加されるべき"
            );
        }

        #[test]
        fn test_spoiler_in_middle_within_limit_after_removal() {
            let message_content = "メッセージの始まり||これは秘密です||メッセージの終わり";
            let message = message_factory(message_content);
            assert_eq!(
                "メッセージの始まりメッセージの終わり",
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value,
                "スポイラーのみが除去され、前後のテキストが結合されるべき"
            );
        }

        #[test]
        fn test_spoiler_containing_url_and_text_then_trimmed() {
            let very_long_text_after_spoiler = "あ".repeat(60);
            let message_content = format!("prefix||spoiler https://example.com spoiler||suffix {very_long_text_after_spoiler}");
            let message = message_factory(&message_content);

            // スポイラーとURL除去後の期待されるベース文字列
            // convert_discord_string("prefix||spoiler https://example.com spoiler||suffix あああ...")
            // -> "prefixsuffix あああ..."
            let expected_base_after_conversion_and_url_removal =
                format!("prefixsuffix {very_long_text_after_spoiler}");

            let expected_final_message = if expected_base_after_conversion_and_url_removal
                .chars()
                .count()
                > 50
            {
                let trimmed = expected_base_after_conversion_and_url_removal
                    .chars()
                    .take(50)
                    .collect::<String>();
                format!("{trimmed}うぬんかんぬん")
            } else {
                expected_base_after_conversion_and_url_removal
            };

            assert_eq!(
                expected_final_message,
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value,
                "スポイラー除去後、URLも除去され、残りが長ければトリムされるべき"
            );
        }
    }

    use regex::Regex;

    #[test]
    fn test_factory() {
        let m = message_factory("message");
        assert_eq!("message", m.msg.content);
    }

    /// テスト用の`Message`オブジェクトを生成します。
    ///
    /// 指定された`content`を持つ`Message`をJSON文字列から作成します。
    fn message_factory(content: &str) -> Message {
        let message_json = r#"{
        "id":881482961801842698,
        "attachments":[],
        "author": {
            "id":502486808211357707,
            "avatar":"bfdafa09852e451e32f7ac1919bab46f",
            "bot":false,
            "discriminator":6539,
            "username":"tkt",
            "public_flags":0
        },
        "channel_id":713052877911752724,
        "content":"[CONTENT]",
        "edited_timestamp":null,
        "embeds":[],
        "guild_id":713052821850816604,
        "type":0,
        "member": {
            "deaf":false,
            "joined_at":"2020-05-21T15:37:20.702Z",
            "mute":false,
            "nick":null,
            "roles":[],
            "pending":false,
            "premium_since":null,
            "guild_id":null,
            "user":null
        },
        "mention_everyone":false,
        "mention_roles":[],
        "mention_channels":[],
        "mentions":[],
        "nonce":"881482961130618880",
        "pinned":false,
        "reactions":[],
        "timestamp":"2021-08-29T10:18:35.255Z",
        "tts":false,
        "webhook_id":null,
        "activity":null,
        "application":null,
        "message_reference":null,
        "flags":0,
        "stickers":[],
        "referenced_message":null
    }"#;
        let re = Regex::new(r"\[CONTENT\]").unwrap();
        let result = re.replace(message_json, content).to_string();
        let m: SerenityMessage = serde_json::from_str(&result[..]).unwrap();
        Message::new(m)
    }
}
