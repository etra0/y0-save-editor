#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::MainWindow) {
  ui->setupUi(this);
  connect(ui->load_button, &QPushButton::clicked, this,
          &MainWindow::on_load_button_clicked);
  connect(ui->save_button, &QPushButton::clicked, this,
          &MainWindow::on_save_button_clicked);
  connect(ui->current_char, &QComboBox::currentTextChanged, this,
          [&](const QString &arg) {
            MainWindow::on_input_changed(arg, "current_char");
          });
  connect(ui->difficulty, &QComboBox::currentTextChanged, this,
          [&](const QString &arg) {
            MainWindow::on_input_changed(arg, "difficulty");
          });
  connect(ui->kiryu_money, &QPlainTextEdit::textChanged, this,
          [&]() { MainWindow::on_text_changed(ui->kiryu_money, "kiryu_money"); });

  QVector<QWidget *> elements_local{ui->save_button, ui->current_char,
                                    ui->difficulty, ui->kiryu_money};
  elements = elements_local;

  for (auto &w : elements) {
    w->setEnabled(false);
  }
}

MainWindow::~MainWindow() { delete ui; }

void MainWindow::initialize_ui_variables() {
  ui->current_char->setCurrentText(
      QString::fromStdString(savefile["current_char"]));
  ui->difficulty->setCurrentText(
      QString::fromStdString(savefile["difficulty"]));

  uint64_t kiryu_money = savefile["kiryu_money"];
  ui->kiryu_money->setPlainText(QString::fromStdString(std::to_string(kiryu_money)));

  for (auto &w : elements) {
    w->setEnabled(true);
  }
}

void MainWindow::on_load_button_clicked() {
  // TODO: remove the starting path
  std::string filename =
      QFileDialog::getOpenFileName(
          this, "Open Image",
          "/Users/etra/Documents/y0-save-editor/backend/tests/",
          "Save Files (*.sav)")
          .toStdString();
  if (filename == "")
    return;

  char *parsed = backend::parse_file(filename.c_str());
  std::string parse_string = parsed;
  backend::free_rust_string(parsed);

  std::cout << "parse_string" << std::endl;

  if (parse_string.rfind("ERR:", 0) == 0) {
    QMessageBox message;
    QString err = "An error ocurred:\n" + QString::fromStdString(parse_string);
    message.setText(err);
    message.exec();
    return;
  }

  this->filename = filename;
  savefile = json::parse(parse_string);
  MainWindow::initialize_ui_variables();
}

void MainWindow::on_save_button_clicked() {
  backend::write_savegame(filename.c_str(), savefile.dump().c_str());
}

void MainWindow::on_input_changed(const QString &arg, const std::string &name) {
  savefile[name] = arg.toStdString();
}

void MainWindow::on_text_changed(const QPlainTextEdit * el, const std::string & name) {
    QString t = el->toPlainText();
    savefile[name] = t.toStdString();

}
