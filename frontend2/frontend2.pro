QT       += core gui

QMAKE_MACOSX_DEPLOYMENT_TARGET += -mmacosx-version-min=10.15

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++17

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    main.cpp \
    mainwindow.cpp

HEADERS += \
    backend.hpp \
    json.hpp \
    mainwindow.h

FORMS += \
    mainwindow.ui

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

win32:CONFIG(release, debug|release): LIBS += -L$$PWD/../backend/target/debug/release/ -lbackend
else:win32:CONFIG(debug, debug|release): LIBS += -L$$PWD/../backend/target/debug/debug/ -lbackend
else:unix: LIBS += -L$$PWD/../backend/target/debug/ -lbackend

INCLUDEPATH += $$PWD/../backend/target/debug
DEPENDPATH += $$PWD/../backend/target/debug
