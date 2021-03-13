# spcy

spcをmp3に変換するDiscord用bot

[here is English README](/README.md)

GitHub Releases でビルド済みのバイナリを配布しています。 https://github.com/naari3/spcy/releases

# 使い方

## Discord botを作成、トークンを取得、サーバーに招待

ref. https://discordpy.readthedocs.io/ja/latest/discord.html

Discord botを作成し、トークンを取得します。

(トークンは次のステップで使用します)

botをDiscordサーバーに招待します。

OAuth scopeの設定は次の画像を参考にしてください。

![scopes](/imgs/discord_bot_oauth_permission.png)

## 起動

`.env.sample` ファイルをコピーし、 `.env` にリネームします。

`.env` ファイルを開き、先程取得したトークンを貼り付けます。

最後に、exeファイルを実行します。