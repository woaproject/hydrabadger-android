// Automaticaly generated by rust_swig
package net.korul.hbbft;
import android.support.annotation.NonNull;

public final class Session {

    public Session()  {

        mNativeObj = init();
    }
    private static native long init() ;

    public final void start_node1()  {

        do_start_node1(mNativeObj);
    }
    private static native void do_start_node1(long me) ;

    public final void start_node2()  {

        do_start_node2(mNativeObj);
    }
    private static native void do_start_node2(long me) ;

    public final void start_node3()  {

        do_start_node3(mNativeObj);
    }
    private static native void do_start_node3(long me) ;

    public final void subscribe1(@NonNull MyObserver1 a0)  {

        do_subscribe1(mNativeObj, a0);
    }
    private static native void do_subscribe1(long me, MyObserver1 a0) ;

    public final void subscribe2(@NonNull MyObserver2 a0)  {

        do_subscribe2(mNativeObj, a0);
    }
    private static native void do_subscribe2(long me, MyObserver2 a0) ;

    public final void subscribe3(@NonNull MyObserver3 a0)  {

        do_subscribe3(mNativeObj, a0);
    }
    private static native void do_subscribe3(long me, MyObserver3 a0) ;

    public final void after_subscribe()  {

        do_after_subscribe(mNativeObj);
    }
    private static native void do_after_subscribe(long me) ;

    public final void change1(boolean a0)  {

        do_change1(mNativeObj, a0);
    }
    private static native void do_change1(long me, boolean a0) ;

    public final void change2(boolean a0)  {

        do_change2(mNativeObj, a0);
    }
    private static native void do_change2(long me, boolean a0) ;

    public final void change3(boolean a0)  {

        do_change3(mNativeObj, a0);
    }
    private static native void do_change3(long me, boolean a0) ;

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