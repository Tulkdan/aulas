package presentation;

public class User {

  public String firstName;
  public String lastName;
  public String email;

  public void create(String firstname, String lastname) throws Exception {
    String providersPath = "providers.csv";
    HelperFile providersFile = new HelperFile();
    providersFile.open(providersPath);
    String provider = providersFile.getRandomItem();

    firstName = firstname;
    lastName = lastname;

    String createEmail = firstname + "." + lastname + "@" + provider;

    email = createEmail;
  }
}
