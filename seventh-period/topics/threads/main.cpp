// Example program
#include <iostream>
#include <thread>
#include <vector>
#include <mutex>
#include <chrono>
#include <atomic>

class Wallet {
private:
  //int _money;
  std::atomic<int> _money;
  //std::mutex mutex;
public:
  Wallet() : _money(0){}
  int getMoney() { return _money; }
  void addMoney(int value) {
    for(int i = 0; i < value; ++i) {
      //mutex.lock();
      _money++;
      //mutex.unlock();
    }
  }
};

int TestExemploTales() {
  Wallet wallet;
  std::vector<std::thread> threads;

  // 5 threads pondo 1000 cada um, para um total de 5000
  for(int i = 0; i < 5; ++i) {
    threads.push_back(std::thread(&Wallet::addMoney, &wallet, 1000));
  }

  // Espera os threads finalizarem
  for(int i = 0; i < 5; ++i) {
    threads[i].join();
  }

  // Retorna 5000
  return wallet.getMoney();
}


int main() {

  auto start = std::chrono::steady_clock::now();

  int moneyCount = 0;
  for(int i = 0; i < 1000; ++i) {
    if((moneyCount = TestExemploTales()) != 5000) {
      std::cout << "Erro na contagem " << i << " pois tem apenas " << moneyCount << " na carteira\n";
    }
  }

  auto end = std::chrono::steady_clock::now();

  std::cout << "Elapsed time in milliseconds : "
    << std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count()
    << " ms" << std::endl;

  std::cout << "Fim do programa\n";
  return 0;
}
