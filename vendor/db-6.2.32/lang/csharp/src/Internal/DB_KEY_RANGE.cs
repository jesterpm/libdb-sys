//------------------------------------------------------------------------------
// <auto-generated />
//
// This file was automatically generated by SWIG (http://www.swig.org).
// Version 3.0.7
//
// Do not make changes to this file unless you know what you are doing--modify
// the SWIG interface file instead.
//------------------------------------------------------------------------------

namespace BerkeleyDB.Internal {

using global::System;
using global::System.Runtime.InteropServices;

internal class DB_KEY_RANGE : global::System.IDisposable {
  private global::System.Runtime.InteropServices.HandleRef swigCPtr;
  protected bool swigCMemOwn;

  internal DB_KEY_RANGE(global::System.IntPtr cPtr, bool cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = new global::System.Runtime.InteropServices.HandleRef(this, cPtr);
  }

  internal static global::System.Runtime.InteropServices.HandleRef getCPtr(DB_KEY_RANGE obj) {
    return (obj == null) ? new global::System.Runtime.InteropServices.HandleRef(null, global::System.IntPtr.Zero) : obj.swigCPtr;
  }

  ~DB_KEY_RANGE() {
    Dispose();
  }

  public virtual void Dispose() {
    lock(this) {
      if (swigCPtr.Handle != global::System.IntPtr.Zero) {
        if (swigCMemOwn) {
          swigCMemOwn = false;
          libdb_csharpPINVOKE.delete_DB_KEY_RANGE(swigCPtr);
        }
        swigCPtr = new global::System.Runtime.InteropServices.HandleRef(null, global::System.IntPtr.Zero);
      }
      global::System.GC.SuppressFinalize(this);
    }
  }

  internal double less {
    get {
      double ret = libdb_csharpPINVOKE.DB_KEY_RANGE_less_get(swigCPtr);
      return ret;
    } 
  }

  internal double equal {
    get {
      double ret = libdb_csharpPINVOKE.DB_KEY_RANGE_equal_get(swigCPtr);
      return ret;
    } 
  }

  internal double greater {
    get {
      double ret = libdb_csharpPINVOKE.DB_KEY_RANGE_greater_get(swigCPtr);
      return ret;
    } 
  }

  internal DB_KEY_RANGE() : this(libdb_csharpPINVOKE.new_DB_KEY_RANGE(), true) {
  }

}

}
