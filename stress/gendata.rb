require 'json'

data = {}

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

  (data[reqs_per_client] ||= []).push(*ttfb.zip(ttc).map {|a,b| [clients,a,b]})
}

for reqs_per_client, items in data
  File.write("data_x#{reqs_per_client}.csv", "clients,ttfb,ttc\n" + 
  items.map{|i| i.join(',')}.join("\n") + "\n")
end
