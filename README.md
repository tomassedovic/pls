# pls

## GUI Development and Deployment Notes

`pls` uses `fbs`, the fman build system:

https://build-system.fman.io/

It's a GPL'd cross-platform GUI development kit using Python and Qt.

### Creating a project

    $ mkdir pls
    $ cd pls
    $ python3 -m venv venv
    $ source venv/bin/activate
    $ pip install fbs PyQt5==5.9.2
    $ fbs startproject
    $ vim src/main/python/main.py
    $ fbs run

### Building a tarball

    $ fbs freeze  # executable is in target/pls/pls
    $ cd target/
    $ tar -czf pls.tgz pls

### Building an RPM

(on a RPM system)

    $ sudo dnf install rubygems ruby-devel rpm-build
    $ gem install -N fpm
    $ fbs installer

### Building a Windows executable

NOTE: `fbs freeze` should work here as well. It's only the installer that
requires NSIS.

https://build-system.fman.io/pyqt-exe-creation/

Unfortunately, this does need a Windows system as well. Sigh. I should get a VM
and try it out though. Shouldn't be terrible.
