require 'json'

puts "x,clients,ttfb,ttc"

Dir["*.json"].each {|f|
  json = JSON.load(File.read(f))
  begin
    clients = json['num_clients']
    reqs_per_client = json['reqs_per_client']
    ttfb = json['time_to_first_byte']
    ttc = json['time_to_completion']
  rescue
    next
  end

  ttfb.zip(ttc).each {|a,b|
    puts [reqs_per_client,clients,a,b].join(",")
  }
}
