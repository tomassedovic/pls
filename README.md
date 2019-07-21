# pls

## GUI Development and Deployment Notes

`pls` uses `fbs`, the fman build system:

https://build-system.fman.io/

It's a GPL'd cross-platform GUI development kit using Python and Qt.

### Running the project

    $ cd pls
    $ python3 -m venv venv
    $ source venv/bin/activate
    $ pip install -r requirements.txt
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

#### Prerequisites:

* Windows (64bit)
* Install Python 64bit
* Install Git for Windows
* Visual C++ Redistributable for Visual Studio 2012 Update 4
  https://www.microsoft.com/en-us/download/details.aspx?id=30679
* Possibly add `C:\Windows\System32`, `C:\Windows\SysWOW64` to PATH

https://build-system.fman.io/pyqt-exe-creation/
