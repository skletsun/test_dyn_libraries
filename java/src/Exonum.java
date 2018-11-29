public class Exonum {

    public native int callExonumBinding(int i);
    public native int callExonumBindingDepExonum(int i);

    public Exonum() {
        System.out.print("Loading library exonum_binding from Java...");
        System.loadLibrary("exonum_binding");
        System.out.println("\tDONE");
    }
}
