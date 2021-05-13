import xml.etree.ElementTree as ET
import sys
import requests

if __name__ == "__main__":
    filename = sys.argv[1]
    server = sys.argv[2]
    username = sys.argv[3]
    password = sys.argv[4]

    response = requests.post(server + "/accounts/ClientLogin", data={"Email": username, "Passwd": password})
    token = response.text.splitlines()[-1][5:]
    headers = {"Authorization": "GoogleLogin auth=" + token}

    root = ET.parse(filename).getroot()
    for body in root.findall("body"):
        for folder in body:
            folder_title = folder.attrib["title"]
            for feed in folder:
                feed_url = feed.attrib["xmlUrl"]
                title = feed.attrib["title"]
                requests.post(server + "/reader/api/0/subscription/quickadd?quickadd=" + feed_url, data={}, headers = headers)
                requests.post(server + "/reader/api/0/subscription/edit", data={"ac": "edit","t": title, "s": "feed/" + feed_url,"a": ["user/-/label/" + folder_title]}, headers = headers)