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
    try:
        files = os.listdir(directory_path)
    except FileNotFoundError:
        files = []
    return natural_sort(files)


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
        raise NotImplementedError(f"Unknown platform {system}")


def load_config_file(config_path):
    config = configparser.ConfigParser()
    config.read(config_path)
    return config


def ensure_config_directory_exists(config_path):
    os.makedirs(config_path.parent, exist_ok=True)


def series_directory(config, series):
    series = series.lower()
    hostname = platform.node().lower()
    hostname_dir_key = f'directory_{hostname}'
    try:
        return Path(config[series][hostname_dir_key])
    except KeyError:
        return Path(config[series]['directory'])


def file_to_play(config, series_id, directory):
    current_filename = config[series_id]['next']
    to_play = Path(directory) / current_filename
    if to_play.exists():
        return to_play
    else:
        return Path(directory) / f"Error #1: File '{current_filename}' Not Found!"


def next_file_to_play(series_directory, current_filename):
    all_files = list_sorted_files(series_directory)
    try:
        current_index = all_files.index(current_filename)
    except ValueError:
        # TODO(shadower): handle this properly. File not found?
        return ""

    try:
        next_filename = all_files[current_index + 1]
    except IndexError:
        # TODO(shadower): handle this properly. Reached the end?
        return ""

    return next_filename


def last_played_file(config, series_id, series_directory):
    all_files = list_sorted_files(series_directory)

    next_filename = config[series_id]['next']
    try:
        current_index = all_files.index(next_filename)
    except ValueError:
        return Path(series_directory) / f"Error #2: File '{next_filename}' Not Found!"

    if current_index == 0:
        # We're at the beginning
        return Path(series_directory) / next_filename

    try:
        last_played_filename = all_files[current_index - 1]
    except IndexError:
        return Path(series_directory) / f"Error #3: No Previous File Exists!"

    return Path(series_directory) / last_played_filename


def play_file(file_path):
    system = platform.system()
    if system == 'Linux':
        # NOTE: `xdg-open` returns immediately
        result = subprocess.run(("xdg-open", file_path))
        if result.returncode != 0:
            print("Error playing video:\n", result)
    elif system == 'Windows':
        os.startfile(file_path)
    elif system == 'Darwin':
        raise NotImplementedError("macOS systems are not supported yet")
    else:
        raise NotImplementedError(f"Unknown platform {system}")


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
            sys.exit(f"Unknown command line option: '{cli_option}'")
    else:
        sys.exit(
            f"Incorrect number of command line arguments: {len(sys.argv) - 1}")

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

    def config(self):
        config_path = config_file_location()
        ensure_config_directory_exists(config_path)
        # TODO: create the config file as well, not just the dir
        assert config_path.parent.exists()
        assert config_path.parent.is_dir()
        config = load_config_file(config_path)
        return config

    def shows(self, config):
        for show_id in config.sections():
            yield self.series(config, show_id)

    def series(self, config, series_id):
        series = Series()
        try:
            series.name = config[series_id]['name']
        except KeyError:
            series.name = series_id
        series.id = series_id
        series.location = series_directory(config, series_id)
        series.last_watched_episode_path = last_played_file(config, series_id, series.location)
        series.next_episode_path = file_to_play(config, series_id, series.location)
        series.episode_after_the_current_one = next_file_to_play(series.location, series.next_episode_path.name)
        return series

    def set_next_and_save(self, config, series):
        next_filename = series.episode_after_the_current_one
        if next_filename:
            config[series.id]['next'] = next_filename
            series.next()
            config_path = config_file_location()
            save_config(config, config_path)


class Series():
    def replay_last_watched(self):
        if self.last_watched_episode_path:
            play_file(self.last_watched_episode_path)
        else:
            print("Can't play the last-watched file. No such file is on the record..")

    def play_next(self):
        if self.next_episode_path:
            play_file(self.next_episode_path)
        else:
            print("Can't play next file. You've reached the end.")

    def next(self):
        self.last_watched_episode_path = self.next_episode_path
        self.next_episode_path = self.location / self.episode_after_the_current_one
        self.episode_after_the_current_one = next_file_to_play(self.location, self.next_episode_path.name)
