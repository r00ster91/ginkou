// TODO: いつかhttps://github.com/rust-lang/rust/issues/55467が実現したらJapaneseを「日本語」にしよう！

use crate::{account_file_name, Language};

pub const JAPANESE: Language = Language {
    introduction: "いらっしゃいませ！WとDキーで選択を選んでください。",
    sign_in: "サインイン",
    sign_up: "サインアップ",
    exit: "出口",
    password_mismatch: "パスワードは合っていません。",
    name_mismatch: "名前は合っていません。",
    registration_finish: "サインアップありがとうございました。",
    file_error: concat!(account_file_name!(), "というファイルができませんでした。"),
    invalid_id: "無効なIDです。IDを作るにはサインアップしてください。",
    id_is: "IDは",
    name: "名前",
    password: "パスワード",
    password_confirmation: "確認のパスワード",
    balance: "残高",
    length_minimum: "の長さの最低は一文字です。",
    deposit: "貯金",
    withdraw: "引き出す",
    delete_account: "アカウントを削除",
    done: "完了",
    amount: "数",
    invalid_amount: "無効な数です。",
    signed_in: "サインインしました。",
    corrupted_account_file: "アカウントのファイルが文字化けになりました。削除してください。",
    write_failed: "保存できませんでした。",
};
