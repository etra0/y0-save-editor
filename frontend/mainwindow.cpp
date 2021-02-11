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
//  connect(ui->kiryu_money, &QLineEdit::textChanged, this,
//          [&](const QString &arg) { MainWindow::on_input_changed(arg, "kiryu_money"); });
//  connect(ui->majima_money, &QLineEdit::textChanged, this,
//          [&](const QString &arg) { MainWindow::on_input_changed(arg, "majima_money"); });

  this->set_global_input(false);

  YakuzaItem *t = new YakuzaItem(ui->scrollArea, 1);



  std::map<std::string, QCheckBox *> outfits = {
      {"dod", ui->outfit_dod},
      {"dragon", ui->outfit_dragon},
      {"producer", ui->outfit_producer},
      {"judgement", ui->outfit_judgement},
      {"new_hire", ui->outfit_new_hire}
  };

  for (auto const & [key, value] : outfits) {
      auto key_copy = key;
      auto value_copy = value;
      // This is maybe a bit unsafe
      auto lambda = [key_copy, value_copy, this](int status) {
          this->savefile["outfit"][key_copy] = value_copy->isChecked();
      };
      connect(value, &QCheckBox::stateChanged, this, lambda);
  }

}

MainWindow::~MainWindow() { delete ui; }

void MainWindow::set_global_input(bool state) {
    ui->general_groupbox->setEnabled(state);
    ui->general_tab->setEnabled(state);
    ui->save_button->setEnabled(state);
}

void MainWindow::initialize_ui_variables() {
  ui->current_char->setCurrentText(
      QString::fromStdString(savefile["current_char"]));
  ui->difficulty->setCurrentText(
      QString::fromStdString(savefile["difficulty"]));

  uint64_t kiryu_money = savefile["kiryu_money"];
  ui->kiryu_money->setText(QString::fromStdString(std::to_string(kiryu_money)));

  uint64_t majima_money = savefile["majima_money"];
  ui->majima_money->setText(QString::fromStdString(std::to_string(majima_money)));

  std::cout << savefile << std::endl;
  // kiryu
  ui->style_dod->setChecked(savefile["style_dod"]);
  ui->style_beast->setChecked(savefile["style_beast"]);
  ui->style_rush->setChecked(savefile["style_rush"]);

  // Majima
  ui->style_mdos->setChecked(savefile["style_mdos"]);
  ui->style_breaker->setChecked(savefile["style_breaker"]);
  ui->style_slugger->setChecked(savefile["style_slugger"]);

  this->set_global_input(true);

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
