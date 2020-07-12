from statistics import mean, stdev

import requests

endpoint = 'https://speed-test.samuelcolvin.workers.dev'


def get() -> int:
    """
    Make a GET request to the worker

    We don't use a session (requests.Session) to avoid reusing a connection and thereby always
    getting the same CloudFlare ray.
    """
    r = requests.get(endpoint, params={'template': 'testing {{ name }}', 'name': 'test'})
    r.raise_for_status()
    time_taken = int(r.elapsed.total_seconds() * 1000)
    print(f'request took {time_taken}ms, response {r.text!r}')
    return time_taken


def main():
    times = []
    for i in range(100):
        t = get()
        times.append(t)

    print(f'mean time {mean(times)}ms, stdev {stdev(times)}ms')


if __name__ == '__main__':
    main()
