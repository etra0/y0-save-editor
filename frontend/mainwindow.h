#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include "backend.hpp"
#include "json.hpp"
#include <QComboBox>
#include <QFileDialog>
#include <QMainWindow>
#include <QMessageBox>
#include <QLineEdit>
#include <QPushButton>
#include <QCheckBox>
#include <QVector>
#include <iostream>
#include <string>
#include <map>
#include "yakuzaitem.h"
#include "itemdialog.h"
#include "utils.h"

using json = nlohmann::json;

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  MainWindow(QWidget *parent = nullptr);
  ~MainWindow();


private slots:
  void on_edit_items_button_clicked();

private:
  void on_load_button_clicked();
  void on_save_button_clicked();

private:
  void initialize_ui_variables();

  // Useful to block input if a save isn't loaded yet.
  void set_global_input(bool state);
//  void on_text_changed(const QPlainTextEdit * el, const std::string & name);

private:
  Ui::MainWindow *ui;
  json savefile;
  std::string filename;
  std::vector<YakuzaItem> items;
};
#endif // MAINWINDOW_H
