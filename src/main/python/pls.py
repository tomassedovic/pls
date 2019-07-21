import configparser
from enum import Enum, auto
import os
from pathlib import Path
import platform
import re
import subprocess
import sys

def natural_sort(l):
    convert = lambda text: int(text) if text.isdigit() else text.lower()
    alphanum_key = lambda key: [ convert(c) for c in re.split('([0-9]+)', key) ]
    return sorted(l, key = alphanum_key)


def list_sorted_files(directory_path):
    return natural_sort(os.listdir(directory_path))


def config_file_location():
    system = platform.system()
    if system == 'Linux':
        xdg_config_home = os.environ.get('XDG_CONFIG_HOME')
        if xdg_config_home:
            base_path = Path(xdg_config_home)
        else:
            base_path = Path.home() / '.config'
        config_path = base_path / 'pls' / 'pls.toml'
        return config_path
    elif system == 'Windows':
        appdata = os.environ['APPDATA']
        base_path = Path(appdata)
        config_path = base_path / 'pls' / 'pls.toml'
        return config_path
    elif system == 'Darwin':
        raise NotImplementedError("macOS systems are not supported yet")
    else:
        raise NotImplementedError("Unknown platform {}".format(system))


def load_config_file(config_path):
    config = configparser.ConfigParser()
    config.read(config_path)
    return config


def ensure_config_directory_exists(config_path):
    os.makedirs(config_path.parent, exist_ok=True)


def add_series(config, series, directory):
    series = series.lower()
    config[series] = {}
    config[series]['directory'] = '/home/thomas/Videos/Bleach/'
    config[series]['next'] = 'Bleach - 18.mkv'


def set_next(config, series, filename):
    series = series.lower()
    config[series]['next'] = filename

def series_directory(config, series):
    series = series.lower()
    return config[series]['directory']

def file_to_play(config, series):
    series = series.lower()
    current_filename = config[series]['next']
    return Path(series_directory(config, series)) / current_filename


def next_file_to_play(config, series):
    series = series.lower()
    current_directory = config[series]['directory']
    all_files = list_sorted_files(current_directory)

    current_filename = config[series]['next']
    try:
        current_index = all_files.index(current_filename)
    except ValueError:
        sys.exit("File {} not found in {}.\nAborting.".format(
            current_filename, current_directory))

    try:
        next_filename = all_files[current_index + 1]
    except IndexError:
        sys.exit("Reached the end of the directory.\nAborting.")

    return next_filename


def last_played_file(config, series):
    series = series.lower()
    current_directory = config[series]['directory']
    all_files = list_sorted_files(current_directory)

    next_filename = config[series]['next']
    try:
        current_index = all_files.index(next_filename)
    except ValueError:
        sys.exit("File {} not found in {}.\nAborting.".format(
            next_filename, current_directory))

    try:
        last_played_filename = all_files[current_index - 1]
    except IndexError:
        sys.exit("Reached the end of the directory.\nAborting.")

    return  Path(current_directory) / last_played_filename


def play_file(file_path):
    # NOTE: `xdg-open` returns immediately
    result = subprocess.run(("xdg-open", file_path))
    if result.returncode != 0:
        print("Error playing video:\n", result)


def save_config(config, config_path):
    with open(config_path, 'w') as config_file:
        config.write(config_file)


class Action(Enum):
    PLAY_NEXT = auto()
    SHOW_LAST = auto()
    SHOW_NEXT = auto()
    PLAY_LAST = auto()

USAGE = '''Usage:
pls\t\t\tPlay the next episode
pls --show-last\t\tShow the name of the last played episode
pls --show-next\t\tShow the next episode to play
'''


def run():
    if len(sys.argv) <= 1:
        action = Action.PLAY_NEXT
    elif len(sys.argv) == 2:
        cli_option = sys.argv[1]
        if cli_option == '--show-last':
            action = Action.SHOW_LAST
        elif cli_option == '--show-next':
            action = Action.SHOW_NEXT
        elif cli_option == '--play-last':
            action = Action.PLAY_LAST
        elif cli_option == '--help':
            print(USAGE)
            sys.exit(0)
        else:
            sys.exit("Unknown command line option: '{}'".format(cli_option))
    else:
        sys.exit("Incorrect number of command line arguments: {}".format(len(sys.argv) - 1))

    config_path = config_file_location()
    ensure_config_directory_exists(config_path)
    # TODO: create the config file as well, not just the dir

    assert config_path.parent.exists()
    assert config_path.parent.is_dir()
    config = load_config_file(config_path)

    # TODO: make the series configurable from CLI
    series = 'Bleach'

    config = load_config_file(config_path)

    if action == Action.PLAY_NEXT:
        print("Configuration:", config_path)
        print("Selected series:", series)
        path = file_to_play(config, series)
        print("Playing file:", path)
        play_file(path)

        next_filename = next_file_to_play(config, series)
        print("Next file to play:", next_filename)
        set_next(config, series, next_filename)
        save_config(config, config_path)
    elif action == Action.PLAY_LAST:
        path = last_played_file(config, series)
        play_file(path)
    elif action == Action.SHOW_NEXT:
        path = file_to_play(config, series)
        print(path)
    elif action == Action.SHOW_LAST:
        path = last_played_file(config, series)
        print(path)


class Pls():
    def __init__(self):
        pass

    def series(self, series_name):
        series_id = series_name.lower()
        config_path = config_file_location()
        ensure_config_directory_exists(config_path)
        # TODO: create the config file as well, not just the dir

        assert config_path.parent.exists()
        assert config_path.parent.is_dir()
        config = load_config_file(config_path)

        series = Series()
        series.name = series_name
        series.id = series_id
        series.location = config[series_id]['directory']
        series.last_watched_episode_path = last_played_file(config, series_id)
        series.next_episode_path = file_to_play(config, series_id)
        return series

    def replay_last_watched(self, series_name):
        path = self.series(series_name).last_watched_episode_path
        play_file(path)

    def play_next(self, series_name):
        path = self.series(series_name).next_episode_path
        play_file(path)

    def set_next(self, series_name):
        next_filename = next_file_to_play(config, series)
        print("Next file to play:", next_filename)
        set_next(config, series, next_filename)

class Series():
    pass
