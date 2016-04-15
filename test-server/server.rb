require 'socket'
require 'irb'
require 'irb/completion'

OK = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n"
NOT_FOUND = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"
ERROR = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n"
REDIRECT = "HTTP/1.1 301 Redirect\r\nLocation: /\r\n\r\n"

@@live = true
@@response = OK
@@sleep = 0
@@redirect = 0

port = ENV.fetch('PORT', 7125)
puts "Starting server at localhost:#{port}"
webserver = TCPServer.new('127.0.0.1', port)
server = Thread.new do
  while (session = webserver.accept)
    puts 'Received request'
    sleep @@sleep
    session.print @@response if @@live
    @@redirect = @@redirect.pred if @@response == REDIRECT
    ok if @@redirect.zero?
    session.close
  end
end


def ok
  @@response = OK
end

def error
  @@response = ERROR
end

def not_found
  @@response = NOT_FOUND
end

def alive
  @@live = true
end

def dead
  @@live = false
end

def redirect times=1
  @@redirect = times
  @@response = REDIRECT
end

def speed(amount)
  @@sleep = amount
end

IRB.start
server.exit
