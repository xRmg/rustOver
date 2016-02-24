# Name
PushOver client in Rust - In progress

# Usage
<pre>
rustOver.

Pushover client in rust.
Create a pushover application in your Pushover Dashboard, grab the token and user token and send a message.
Take a look at -h for extra options

Usage:
    rustOver  &lt;token &gt;  &lt;user-token &gt;  &lt;message &gt; [options]
    rustOver (-h | --help)

Options
  --title= &lt;title &gt;          Title of the message, otherwise the App name is used.
  --devices= &lt;devices &gt;      List of devices to send the message to, seperated by comma.
  --url= &lt;url &gt;              Supplementary url to show with your message.
  --url-title= &lt;url_title &gt;  A title for your supplementary URL, otherwise just the url is shown.
  --priority= &lt;n &gt;           Priority of the message -2..2 (low..high).
  --timestamp= &lt;unix_t &gt;     Unix timestamp to be added to the message.
  --sound= &lt;sound &gt;          Sound to be played on receiving the message.
  --use-html               Enable the usage of HTML in Message.
  -h --help     Show this screen.
  -v            Verbose mode
</pre>

#License

The MIT License (MIT)
