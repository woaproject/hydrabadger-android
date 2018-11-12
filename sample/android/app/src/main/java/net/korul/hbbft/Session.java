// Automaticaly generated by rust_swig
package net.korul.hbbft;
import android.support.annotation.NonNull;

public final class Session {

    public Session()  {

        mNativeObj = init();
    }
    private static native long init() ;

    public final void send_message(int a0, @NonNull String a1)  {

        do_send_message(mNativeObj, a0, a1);
    }
    private static native void do_send_message(long me, int a0, String a1) ;

    public final void start_node(@NonNull String a0, @NonNull String a1, @NonNull String a2)  {

        do_start_node(mNativeObj, a0, a1, a2);
    }
    private static native void do_start_node(long me, String a0, String a1, String a2) ;

    public final void subscribe(@NonNull MyObserver a0)  {

        do_subscribe(mNativeObj, a0);
    }
    private static native void do_subscribe(long me, MyObserver a0) ;

    public final void after_subscribe()  {

        do_after_subscribe(mNativeObj);
    }
    private static native void do_after_subscribe(long me) ;

    public final void change(int a0, boolean a1, @NonNull String a2, @NonNull String a3)  {

        do_change(mNativeObj, a0, a1, a2, a3);
    }
    private static native void do_change(long me, int a0, boolean a1, String a2, String a3) ;

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ long mNativeObj;
}