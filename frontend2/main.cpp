#include "mainwindow.h"
#include "backend.hpp"
#include "json.hpp"
#include <iostream>

#include <QApplication>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    MainWindow w;
    w.show();
    return a.exec();
}
