"""
gist-cli.py
author(s): khuynh
description: a python script that allows you to publish gists from the
terminal cli.
"""

import json
import os
import sys

from pprint import pprint
from getpass import getpass
import requests

# URI = 'https://api.github.com/gists'
URI = 'http://localhost:7878'

if __name__ == '__main__':
    # ignore self filename
    sys.argv = sys.argv[1:]
    print(sys.argv)

    # build json for FILES included in argv
    FILES = {}
    for arg in sys.argv:
        if arg in os.listdir('.'):
            f_str = ''
            content = {}
            with open(arg, 'r') as f:
                for line in f:
                    f_str += line
                content['content'] = f_str
            FILES[arg] = content

    # prompt user for authentication
    USERNAME = input('Enter your USERNAME: ')
    PASSWORD = getpass('Password: ')

    DESCRIPTION = input('Description (Press enter to skip): ')

    # build json to send via POST request
    DATA = json.dumps({
        'description': DESCRIPTION,
        'public': False if '--private' in sys.argv else True,
        'files': FILES,
    })

    R = requests.post(URI, data=DATA, auth=(USERNAME, PASSWORD))

    # handle errors
    if R.status_code != 201:
        errmsg = json.loads(R.content)['message']
        print(f'ERROR: {errmsg}')

    # on success, print files uploaded and url
    if R.status_code == 201:
        content = json.loads(R.content)
        print('Success...\n\n')
        for fn in content['files']:
            raw_url = content['files'][fn]['raw_url']
            gist_id = raw_url.split(USERNAME)[1].split('/')[1]
            url = f'https://gist.github.com/{USERNAME}/{gist_id}'
            new_file = {
                'filename': content['files'][fn]['filename'],
                'url': url,
            }
            print('{filename:<25} {url:>}'.format(**new_file))
