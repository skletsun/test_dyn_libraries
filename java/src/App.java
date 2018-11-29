public class App {
    public static void main(String[] args) {
        // Make sure libraries are reloaded on every test
        switch (args[0]) {
            case "test1": checkExonumAndTestkitLoading();
            break;
            case "test2": checkTestkitAndExonumLoading();
            break;
        }
    }

    private static void checkExonumAndTestkitLoading() {
        System.out.println("---------- First load exonum then testkit ----------");
        checkExonum();
        checkTestkit();
    }

    private static void checkTestkitAndExonumLoading() {
        System.out.println("---------- First load testkit then exonum ----------");
        checkTestkit();
        checkExonum();
    }

    private static void checkTestkit() {
        Testkit testkit = new Testkit();

        System.out.println("java > testkit-binding: " + testkit.callTestkitBinding(1));
        System.out.println("java > testkit-binding > testkit : " + testkit.callTestkitBindingDepTestkit(1));
        System.out.println("java > testkit-binding > testkit > exonum: " + testkit.callTestkitBindingDepTestkitExonum(1));
    }

    private static void checkExonum() {
        Exonum exonum = new Exonum();

        System.out.println("java > exonum-binding: " + exonum.callExonumBinding(1));
        System.out.println("java > exonum-binding > exonum: " + exonum.callExonumBindingDepExonum(1));
    }
}