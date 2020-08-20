#include <iostream>
#include <thread>
#include <mutex>
#include <vector>
#include <chrono>

#define FORKS 5
#define EXECUTIONS 500000

class Philosopher {
  public:
    Philosopher(int id, std::vector<std::mutex>& table) :
      philosopher_id(id),
      table(table) {}

    void exec() {
      for (int i = 0; i < EXECUTIONS; i++) {
        think(philosopher_id);
        eat(philosopher_id, table);
      }
    }
    int philosopher_id;
  private:
    std::vector<std::mutex>& table;

    void think(int philosopher) {
      std::cout << "Philosopher " << philosopher << "thinking...\n";
      std::this_thread::sleep_for(std::chrono::milliseconds(5000));
    }

    void eat(int philosopher, std::vector<std::mutex>& table) {
      int left = philosopher;
      int right = (philosopher + 1) % FORKS;

      std::lock(table[left], table[right]);

      std::lock_guard<std::mutex> left_fork(table[left], std::adopt_lock);
      std::lock_guard<std::mutex> right_fork(table[right], std::adopt_lock);

      std::cout << "Philospher " << philosopher << "eating...\n";
      std::this_thread::sleep_for(std::chrono::milliseconds(2000));

      std::cout << "Philospher " << philosopher << "released forks\n";
    }
};

int main() {
  std::vector<std::mutex> table (FORKS);
  std::vector<std::thread> threads;

  for (int i = 0; i < FORKS; i++) {
    Philosopher philo = Philosopher(i, table);
    threads.push_back(std::thread(&Philosopher::exec, philo));
  }

  for (int i = 0; i < FORKS; i++) {
    threads[i].join();
  }
  return 0;
}
