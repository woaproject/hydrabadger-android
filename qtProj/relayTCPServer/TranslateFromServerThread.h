#ifndef TRANSLATEFROMSERVERTHREAD_H
#define TRANSLATEFROMSERVERTHREAD_H

#include <QObject>
#include <QThread>
#include <QTcpSocket>
#include <QDataStream>
#include <memory>

#include "Accessor.h"

class TranslateFromServerThread : public QThread
{
    Q_OBJECT

    Q_PROPERTY(bool StopThread READ StopThread WRITE setStopThread NOTIFY StopThreadChanged)

public:
    explicit TranslateFromServerThread(qintptr socketDescriptor, QObject *parent = nullptr);
    ~TranslateFromServerThread();

    bool StopThread() const;
    void run() override;

private slots:
    void displayError(QAbstractSocket::SocketError socketError);

signals:
    void StopThreadChanged(bool StopThread);

    void getSocketWithDescriptor(qintptr socketDescriptor, bool fromto);
    void sendData(quint16 PORTFROMLISTEN, QByteArray block, const char *data, int len);
public slots:
    void setStopThread(bool StopThread);

    void initSocket(std::shared_ptr<QTcpSocket> tcpSocket);
    void setUnblock();
    void unblock();

    void disconnect();

private:
    qintptr socketDescriptor;
    bool m_StopThread;

    quint16 localPort{};

    std::shared_ptr<QTcpSocket> tcpSocket;
    bool mIsinit{};
    bool mIsBlockSend{};

    //inputstream
    QDataStream in;
};

#endif // TRANSLATEFROMSERVERTHREAD_H
