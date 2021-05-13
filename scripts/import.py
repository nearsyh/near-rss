import xml.etree.ElementTree as ET
import sys
import requests

if __name__ == "__main__":
    filename = sys.argv[1]
    server = sys.argv[2]

    root = ET.parse(filename).getroot()
    for body in root.findall("body"):
        for folder in body:
            title = folder.attrib["title"]
            for feed in folder:
                feed_url = feed.attrib["xmlUrl"]
                requests.post(server + "/reader/api/0/subscription/quickadd?quickadd=" + feed_url, data={})
                requests.post(server + "/reader/api/0/subscription/edit", data={"ac": "edit","s": "feed/" + feed_url,"a": ["user/-/label/" + title]})