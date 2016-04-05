# Small interactive server

This is a simple interactive response server, where you can change the response from a REPL.

## How to use

```
$ ruby -W0 server.rb

irb> ok
# will set the server to respond with 200
irb> error
# will set the server to respond with 500
irb> not_found
# will set the server to respond with 404
irb> dead
# will just close the connection
irb> alive
# will respond again
irb> exit
# will exit the application
```
