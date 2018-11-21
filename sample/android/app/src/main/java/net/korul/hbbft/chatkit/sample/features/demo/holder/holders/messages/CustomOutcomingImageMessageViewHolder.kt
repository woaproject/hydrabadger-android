package net.korul.hbbft.chatkit.sample.features.demo.holder.holders.messages

import android.util.Pair
import android.view.View

import com.stfalcon.chatkit.messages.MessageHolders
import net.korul.hbbft.chatkit.sample.common.data.model.Message

/*
 * Created by troy379 on 05.04.17.
 */
class CustomOutcomingImageMessageViewHolder(itemView: View, payload: Any) :
    MessageHolders.OutcomingImageMessageViewHolder<Message>(itemView, payload) {

    override fun onBind(message: Message) {
        super.onBind(message)

        time.text = message.status + " " + time.text
    }

    //Override this method to have ability to pass custom data in ImageLoader for loading image(not avatar).
    override fun getPayloadForImageLoader(message: Message): Any? {
        //For example you can pass size of placeholder before loading
        return Pair(100, 100)
    }
}