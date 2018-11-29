public class Testkit {

    public native int callTestkitBinding(int i);
    public native int callTestkitBindingDepTestkit(int i);
    public native int callTestkitBindingDepTestkitExonum(int i);

    public Testkit() {
        System.out.print("Loading library testkit_binding from Java...");
        System.loadLibrary("testkit_binding");
        System.out.println("\tDONE");
    }
}
