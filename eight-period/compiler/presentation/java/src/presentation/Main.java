package presentation;

import java.util.ArrayList;

public class Main {

  public static void main(String[] args) throws Exception {
    String namesPath = "names.csv";
    HelperFile nameFile = new HelperFile();
    nameFile.open(namesPath);

    String pronomePath = "pronomes.csv";
    HelperFile pronomeFile = new HelperFile();
    pronomeFile.open(pronomePath);

    ArrayList<User> users = new ArrayList<User>();

    for (String name : nameFile.data) {
      String lastName = pronomeFile.getRandomItem();
      User user = new User();
      user.create(name, lastName);
      users.add(user);
    }

    for (User user : users) {
      System.out.println("firstName: " + user.firstName + "\tLastName: " + user.lastName + "\tEmail: " + user.email);
    }
  }

}
