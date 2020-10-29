import configparser
from enum import Enum, auto
import os
from pathlib import Path
import platform
import re
import subprocess
import sys


class Error(Exception):
    def __init__(self, code, message):
        self.code = code
        self.message = message

    def __str__(self):
        return f"Error #{self.code}: {self.message}"


def natural_sort(l):
    convert = lambda text: int(str(text)) if str(text).isdigit() else str(text).lower()
    alphanum_key = lambda key: [convert(c) for c in re.split('([0-9]+)', str(key))]
    return sorted(l, key = alphanum_key)


def list_sorted_files(directory_path):
    result = [p for p in directory_path.glob('**/*') if p.is_file()]
    return natural_sort(result)


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
        # https://developer.apple.com/library/archive/documentation/General/Conceptual/MOSXAppProgrammingGuide/AppRuntime/AppRuntime.html
        base_path = Path.home() / 'Library' / 'Application Support'
        config_path = base_path / 'pls' / 'pls.toml'
        return config_path
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
    to_play = Path(directory) / normalise_path_separators(current_filename)
    if to_play.exists():
        return to_play
    else:
        return Error(1, f"File Not Found:\n'{current_filename}'")


def next_file_to_play(series_directory, current_filename):
    all_files = list_sorted_files(series_directory)
    current_path = Path(series_directory, normalise_path_separators(current_filename))
    try:
        current_index = all_files.index(current_path)
    except ValueError:
        # TODO(shadower): handle this properly. File not found?
        return ""

    try:
        next_path = all_files[current_index + 1]
    except IndexError:
        # TODO(shadower): handle this properly. Reached the end?
        return ""

    return next_path

def normalise_path_separators(p):
    '''
    In case of nested dirs, Windows will save path with a backslash
    but unix expects a forward slash.

    This will turn both slashes in a path into the platform-appropriate one.
    '''
    return Path(str(p).replace('\\', os.path.sep).replace('/', os.path.sep))


def last_played_file(config, series_id, series_directory):
    all_files = list_sorted_files(series_directory)

    next_path = Path(series_directory, normalise_path_separators(config[series_id]['next']))
    try:
        current_index = all_files.index(next_path)
    except ValueError:
        return Error(2, f"File Not Found:\n'{next_path.name}'")

    if current_index == 0:
        # We're at the beginning
        # TODO: handle this differently? Show a message instead?
        return next_path

    try:
        last_played_path = all_files[current_index - 1]
    except IndexError:
        return Error(3, "No Previous File Exists!")

    return normalise_path_separators(last_played_path)


def play_file(file_path):
    """Open the file using standard method for the platform."""
    system = platform.system()
    if system == 'Linux':
        # NOTE: `xdg-open` returns immediately
        result = subprocess.run(("xdg-open", file_path))
        if result.returncode != 0:
            print("Error playing video:\n", result)
    elif system == 'Windows':
        os.startfile(file_path)
    elif system == 'Darwin':
        # NOTE: `open` returns immediately by default
        result = subprocess.run(("open", file_path))
        if result.returncode != 0:
            print("Error playing video:\n", result)
    else:
        raise NotImplementedError(f"Unknown platform {system}")


def save_config(config, config_path):
    with open(config_path, 'w') as config_file:
        config.write(config_file)


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
            try:
                yield self.series(config, show_id)
            except Exception as e:
                print(f"Error loading show {show_id}: {repr(e)}")
                pass

    def series(self, config, series_id):
        series = Series()
        try:
            series.name = config[series_id]['name']
        except KeyError:
            series.name = series_id
        series.id = series_id
        series.location = series_directory(config, series_id)
        # TODO: rename these to: `previous`, `current` and `upcoming`?
        # or: `last_played`, `to_play` and `upcoming`?
        # TODO: generate them in the same function call (they're exercising the same data and logic)
        # ALSO: maybe store only the "episode names" instead of the full path?
        #Since they can have the season prefix.
        series.last_watched_episode_path = last_played_file(config, series_id, series.location)
        series.next_episode_path = file_to_play(config, series_id, series.location)
        if isinstance(series.next_episode_path, Error):
            series.episode_after_the_current_one = Error(
                4, f"Preceeding episode is error: {series.next_episode_path}")
        else:
            series.episode_after_the_current_one = next_file_to_play(series.location, series.next_episode_path.relative_to(series.location))
        return series

    def set_next_and_save(self, config, series):
        if not series.episode_after_the_current_one:
            return
        next_filename = series.episode_after_the_current_one.relative_to(series.location)
        if next_filename:
            config[series.id]['next'] = str(next_filename)
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
        self.next_episode_path = self.episode_after_the_current_one
        self.episode_after_the_current_one = next_file_to_play(self.location, self.next_episode_path.relative_to(self.location))


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
